use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures::{future, stream::Stream, sync::oneshot, Future};
use hyper::{self, server::conn::Http, service::Service, Body, Method, Request, Response, StatusCode};
use log::error;
use route_recognizer::Router;
use tokio::net::TcpListener;

use crate::{
    config::ConfigPtr,
    db::{AccessoryList, DatabasePtr},
    event::{EmitterPtr, Event},
    protocol::IdPtr,
    transport::{
        http::{
            event_response,
            handler::{self, accessories, characteristics, identify, pair_setup, pair_verify, pairings},
            status_response,
            EventObject,
        },
        tcp::{EncryptedStream, Session, StreamWrapper},
    },
    Error,
};

enum Route {
    Get(Box<Mutex<dyn handler::Handler + Send>>),
    Post(Box<Mutex<dyn handler::Handler + Send>>),
    GetPut {
        _get: Box<Mutex<dyn handler::Handler + Send>>,
        _put: Box<Mutex<dyn handler::Handler + Send>>,
    },
}

struct Api {
    controller_id: IdPtr,
    event_subscriptions: EventSubscriptions,
    config: ConfigPtr,
    database: DatabasePtr,
    accessories: AccessoryList,
    event_emitter: EmitterPtr,
    router: Arc<Router<Route>>,
}

impl Api {
    fn new(
        controller_id: IdPtr,
        event_subscriptions: EventSubscriptions,
        config: ConfigPtr,
        database: DatabasePtr,
        accessories: AccessoryList,
        event_emitter: EmitterPtr,
        session_sender: oneshot::Sender<Session>,
    ) -> Api {
        let mut router = Router::new();
        router.add(
            "/pair-setup",
            Route::Post(Box::new(Mutex::new(handler::TlvHandlerType::from(
                pair_setup::PairSetup::new(),
            )))),
        );
        router.add(
            "/pair-verify",
            Route::Post(Box::new(Mutex::new(handler::TlvHandlerType::from(
                pair_verify::PairVerify::new(session_sender),
            )))),
        );
        router.add(
            "/accessories",
            Route::Get(Box::new(Mutex::new(handler::JsonHandlerType::from(
                accessories::Accessories::new(),
            )))),
        );
        router.add("/characteristics", Route::GetPut {
            _get: Box::new(Mutex::new(handler::JsonHandlerType::from(
                characteristics::GetCharacteristics::new(),
            ))),
            _put: Box::new(Mutex::new(handler::JsonHandlerType::from(
                characteristics::UpdateCharacteristics::new(),
            ))),
        });
        router.add(
            "/pairings",
            Route::Post(Box::new(Mutex::new(handler::TlvHandlerType::from(
                pairings::Pairings::new(),
            )))),
        );
        router.add(
            "/identify",
            Route::Post(Box::new(Mutex::new(handler::JsonHandlerType::from(
                identify::Identify::new(),
            )))),
        );

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
    type Error = Error;
    type Future = Box<dyn Future<Item = Response<Self::ResBody>, Error = Self::Error> + Send>;
    type ReqBody = Body;
    type ResBody = Body;

    fn call(&mut self, req: Request<Self::ReqBody>) -> Self::Future {
        let (parts, body) = req.into_parts();
        let router = self.router.clone();
        let controller_id = self.controller_id.clone();
        let event_subscriptions = self.event_subscriptions.clone();
        let config = self.config.clone();
        let database = self.database.clone();
        let accessories = self.accessories.clone();
        let event_emitter = self.event_emitter.clone();

        Box::new(
            body.fold(vec![], |mut v, c| {
                v.extend(c.to_vec());
                future::ok::<Vec<u8>, hyper::Error>(v)
            })
            .map_err(|e| e.into())
            .and_then(move |body| {
                if let Ok(route_match) = router.recognize(parts.uri.path()) {
                    match (route_match.handler, parts.method) {
                        (&Route::Get(ref handler), Method::GET) => handler.lock().unwrap().handle(
                            parts.uri,
                            body.into(),
                            &controller_id,
                            &event_subscriptions,
                            &config,
                            &database,
                            &accessories,
                            &event_emitter,
                        ),
                        (&Route::Post(ref handler), Method::POST) => handler.lock().unwrap().handle(
                            parts.uri,
                            body.into(),
                            &controller_id,
                            &event_subscriptions,
                            &config,
                            &database,
                            &accessories,
                            &event_emitter,
                        ),
                        (&Route::GetPut { ref _get, ref _put }, Method::GET) => _get.lock().unwrap().handle(
                            parts.uri,
                            body.into(),
                            &controller_id,
                            &event_subscriptions,
                            &config,
                            &database,
                            &accessories,
                            &event_emitter,
                        ),
                        (&Route::GetPut { ref _get, ref _put }, Method::PUT) => _put.lock().unwrap().handle(
                            parts.uri,
                            body.into(),
                            &controller_id,
                            &event_subscriptions,
                            &config,
                            &database,
                            &accessories,
                            &event_emitter,
                        ),
                        _ => Box::new(future::result(status_response(StatusCode::BAD_REQUEST))),
                    }
                } else {
                    Box::new(future::result(status_response(StatusCode::NOT_FOUND)))
                }
            }),
        )
    }
}

pub type EventSubscriptions = Arc<Mutex<Vec<(u64, u64)>>>;

pub fn serve(
    socket_addr: &SocketAddr,
    config: &ConfigPtr,
    database: &DatabasePtr,
    accessories: &AccessoryList,
    event_emitter: &EmitterPtr,
) -> Result<(), Error> {
    let listener = TcpListener::bind(socket_addr)?;

    let config = config.clone();
    let database = database.clone();
    let accessories = accessories.clone();
    let event_emitter = event_emitter.clone();

    let server = listener
        .incoming()
        .for_each(move |stream| {
            let (encrypted_stream, stream_incoming, stream_outgoing, session_sender) = EncryptedStream::new(stream);
            let stream_wrapper = StreamWrapper::new(stream_incoming, stream_outgoing.clone());
            let event_subscriptions = Arc::new(Mutex::new(vec![]));
            let api = Api::new(
                encrypted_stream.controller_id.clone(),
                event_subscriptions.clone(),
                config.clone(),
                database.clone(),
                accessories.clone(),
                event_emitter.clone(),
                session_sender,
            );
            let http = Http::new();

            event_emitter
                .lock()
                .expect("couldn't add listener for characteristic value change events")
                .add_listener(Box::new(move |event| match *event {
                    Event::CharacteristicValueChanged { aid, iid, ref value } => {
                        let mut dropped_subscriptions = vec![];
                        for (i, &(s_aid, s_iid)) in event_subscriptions
                            .lock()
                            .expect("couldn't read event subscriptions")
                            .iter()
                            .enumerate()
                        {
                            if s_aid == aid && s_iid == iid {
                                let event = EventObject {
                                    aid,
                                    iid,
                                    value: value.clone(),
                                };
                                let event_res = event_response(vec![event]).expect("couldn't create event response");
                                if stream_outgoing.unbounded_send(event_res).is_err() {
                                    dropped_subscriptions.push(i);
                                }
                            }
                        }
                        let mut ev = event_subscriptions.lock().expect("couldn't modify event subscriptions");
                        for s in dropped_subscriptions {
                            ev.remove(s);
                        }
                    },
                    _ => {},
                }));

            tokio::spawn(encrypted_stream.map(|_| ()).map_err(|e| error!("{}", e)));
            tokio::spawn(
                http.serve_connection(stream_wrapper, api)
                    .map(|_| ())
                    .map_err(|e| error!("{}", e)),
            );
            Ok(())
        })
        .map_err(|e| error!("{}", e));

    tokio::run(server);
    Ok(())
}
