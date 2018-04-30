use std::{net::SocketAddr, sync::{Arc, Mutex}, cell::RefCell};

use hyper::{self, server::{Http, Request, Response, Service}, StatusCode, Method};
use futures::{future, Future, stream::Stream, sync::oneshot};
use route_recognizer::Router;
use tokio_core::{net::TcpListener, reactor::Core};
use uuid::Uuid;

use transport::http::{
    handlers::{self, pair_setup, pair_verify, accessories, characteristics, pairings, identify},
    encrypted_stream::{EncryptedStream, Session},
    status_response,
};
use db::{accessory_list::AccessoryList, database::DatabasePtr};
use config::ConfigPtr;
use event::{Event, EmitterPtr};

enum Route {
    Get(Box<RefCell<handlers::Handler>>),
    Post(Box<RefCell<handlers::Handler>>),
    GetPut {
        _get: Box<RefCell<handlers::Handler>>,
        _put: Box<RefCell<handlers::Handler>>,
    },
}

struct Api {
    controller_id: Arc<Option<Uuid>>,
    event_subscriptions: EventSubscriptions,
    config: ConfigPtr,
    database: DatabasePtr,
    accessories: AccessoryList,
    event_emitter: EmitterPtr,
    router: Arc<Router<Route>>,
}

impl Api {
    fn new(
        controller_id: Arc<Option<Uuid>>,
        event_subscriptions: EventSubscriptions,
        config: ConfigPtr,
        database: DatabasePtr,
        accessories: AccessoryList,
        event_emitter: EmitterPtr,
        session_sender: oneshot::Sender<Session>,
    ) -> Api {
        let mut router = Router::new();
        router.add("/pair-setup", Route::Post(
            Box::new(RefCell::new(
                handlers::TlvHandlerType::from(pair_setup::PairSetup::new())
            ))
        ));
        router.add("/pair-verify", Route::Post(
            Box::new(RefCell::new(
                handlers::TlvHandlerType::from(pair_verify::PairVerify::new(session_sender))
            ))
        ));
        router.add("/accessories", Route::Get(
            Box::new(RefCell::new(
                handlers::JsonHandlerType::from(accessories::Accessories::new())
            ))
        ));
        router.add("/characteristics", Route::GetPut {
            _get: Box::new(RefCell::new(
                handlers::JsonHandlerType::from(characteristics::GetCharacteristics::new())
            )),
            _put: Box::new(RefCell::new(
                handlers::JsonHandlerType::from(characteristics::UpdateCharacteristics::new())
            )),
        });
        router.add("/pairings", Route::Post(
            Box::new(RefCell::new(
                handlers::TlvHandlerType::from(pairings::Pairings::new())
            ))
        ));
        router.add("/identify", Route::Post(
            Box::new(RefCell::new(
                handlers::JsonHandlerType::from(identify::Identify::new())
            ))
        ));

        Api {
            controller_id,
            event_subscriptions,
            config,
            database,
            accessories,
            event_emitter,
            router: Arc::new(router),
        }
    }
}

impl Service for Api {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Response, Error=hyper::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let (method, uri, _, _, body) = req.deconstruct();
        let router = self.router.clone();
        let controller_id = self.controller_id.clone();
        let event_subscriptions = self.event_subscriptions.clone();
        let config = self.config.clone();
        let database = self.database.clone();
        let accessories = self.accessories.clone();
        let event_emitter = self.event_emitter.clone();

        Box::new(body.fold(vec![], |mut v, c| {
            v.extend(c.to_vec());
            future::ok::<Vec<u8>, hyper::Error>(v)
        }).and_then(move |body| {
            if let Ok(route_match) = router.recognize(uri.path()) {
                match (route_match.handler, method) {
                    (&Route::Get(ref handler), Method::Get) => handler.borrow_mut()
                        .handle(
                            uri,
                            body,
                            controller_id,
                            &event_subscriptions,
                            &config,
                            &database,
                            &accessories,
                            &event_emitter,
                        ),
                    (&Route::Post(ref handler), Method::Post) => handler.borrow_mut()
                        .handle(
                            uri,
                            body,
                            controller_id,
                            &event_subscriptions,
                            &config,
                            &database,
                            &accessories,
                            &event_emitter,
                        ),
                    (&Route::GetPut { ref _get, ref _put }, Method::Get) => _get.borrow_mut()
                        .handle(
                            uri,
                            body,
                            controller_id,
                            &event_subscriptions,
                            &config,
                            &database,
                            &accessories,
                            &event_emitter,
                        ),
                    (&Route::GetPut { ref _get, ref _put }, Method::Put) => _put.borrow_mut()
                        .handle(
                            uri,
                            body,
                            controller_id,
                            &event_subscriptions,
                            &config,
                            &database,
                            &accessories,
                            &event_emitter,
                        ),
                    _ => Box::new(future::ok(status_response(StatusCode::BadRequest))),
                }
            } else {
                Box::new(future::ok(status_response(StatusCode::NotFound)))
            }
        }))
    }
}

pub type EventSubscriptions = Arc<Mutex<Vec<(u64, u64)>>>;

pub fn serve(
    socket_addr: &SocketAddr,
    config: ConfigPtr,
    database: DatabasePtr,
    accessories: AccessoryList,
    event_emitter: EmitterPtr,
) {
    let mut evt_loop = Core::new().unwrap();
    let listener = TcpListener::bind(socket_addr, &evt_loop.handle()).unwrap();
    let http: Http<hyper::Chunk> = Http::new();
    let handle = evt_loop.handle();

    let server = listener.incoming().for_each(|(stream, _)| {
        let (stream, sender) = EncryptedStream::new(stream);
        let controller_id = Arc::new(stream.controller_id);
        let event_subscriptions = Arc::new(Mutex::new(vec![]));
        let api = Api::new(
            controller_id,
            event_subscriptions.clone(),
            config.clone(),
            database.clone(),
            accessories.clone(),
            event_emitter.clone(),
            sender,
        );

        let event_subscriptions = event_subscriptions.clone();
        event_emitter.lock().unwrap().add_listener(Box::new(move |event| {
            match event {
                &Event::CharacteristicValueChanged { aid, iid } => {
                    let es = event_subscriptions.lock().unwrap();
                    for &(s_aid, s_iid) in es.iter() {
                        if s_aid == aid && s_iid == iid {
                            // TODO - find out why this fires too often
                            println!("Event triggered for aid: {} iid: {}", aid, iid);
                        }
                    }
                },
                _ => {},
            }
        }));

        handle.spawn(http.serve_connection(stream, api).map_err(|_| ()).map(|_| ()));
        Ok(())
    });
    evt_loop.run(server).unwrap();
}
