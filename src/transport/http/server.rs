use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use hyper::server::{Http, Request, Response, Service};
use hyper::{self, StatusCode, Method};
use futures::{future, Future};
use futures::stream::Stream;
use futures::sync::oneshot;
use route_recognizer::Router;
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;
use uuid::Uuid;

use transport::http::handlers::{self, pair_setup, pair_verify, accessories, characteristics, pairings, identify};
use transport::http::encrypted_stream::{EncryptedStream, Session};
use db::accessory_list::AccessoryList;
use db::storage::Storage;
use db::database::Database;
use config::Config;

enum Route<S: Storage> {
    Get(Box<RefCell<handlers::Handler<S>>>),
    Post(Box<RefCell<handlers::Handler<S>>>),
    GetPut {
        get: Box<RefCell<handlers::Handler<S>>>,
        put: Box<RefCell<handlers::Handler<S>>>,
    },
}

struct Api<S: Storage> {
    controller_id: Arc<Option<Uuid>>,
    config: Arc<Config>,
    database: Arc<Mutex<Database<S>>>,
    accessories: AccessoryList,
    router: Arc<Router<Route<S>>>,
}

impl<S: 'static + Storage> Api<S> {
    fn new(
        controller_id: Arc<Option<Uuid>>,
        config: Arc<Config>,
        database: Arc<Mutex<Database<S>>>,
        accessories: AccessoryList,
        session_sender: oneshot::Sender<Session>,
    ) -> Api<S> {
        let mut router = Router::new();
        router.add("/pair-setup", Route::Post(
            Box::new(RefCell::new(handlers::TlvHandlerType::from(pair_setup::PairSetup::new())))
        ));
        router.add("/pair-verify", Route::Post(
            Box::new(RefCell::new(handlers::TlvHandlerType::from(pair_verify::PairVerify::new(session_sender))))
        ));
        router.add("/accessories", Route::Get(
            Box::new(RefCell::new(accessories::Accessories::new()))
        ));
        router.add("/characteristics", Route::GetPut {
            get: Box::new(RefCell::new(characteristics::GetCharacteristics::new())),
            put: Box::new(RefCell::new(characteristics::UpdateCharacteristics::new())),
        });
        router.add("/pairings", Route::Post(
            Box::new(RefCell::new(handlers::TlvHandlerType::from(pairings::Pairings::new())))
        ));
        router.add("/identify", Route::Post(
            Box::new(RefCell::new(identify::Identify::new()))
        ));

        Api { controller_id, config, database, accessories, router: Arc::new(router) }
    }
}

impl<S: 'static + Storage> Service for Api<S> {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Response, Error=hyper::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let (method, uri, _, _, body) = req.deconstruct();
        let router = self.router.clone();
        let controller_id = self.controller_id.clone();
        let database = self.database.clone();
        let accessories = self.accessories.clone();

        Box::new(body.fold(vec![], |mut v, c| {
            v.extend(c.to_vec());
            future::ok::<Vec<u8>, hyper::Error>(v)
        }).and_then(move |body| {
            if let Ok(route_match) = router.recognize(uri.path()) {
                match (route_match.handler, method) {
                    (&Route::Get(ref handler), Method::Get) => handler.borrow_mut()
                        .handle(uri, body, controller_id, &database, &accessories),
                    (&Route::Post(ref handler), Method::Post) => handler.borrow_mut()
                        .handle(uri, body, controller_id, &database, &accessories),
                    (&Route::GetPut { ref get, ref put }, Method::Get) => get.borrow_mut()
                        .handle(uri, body, controller_id, &database, &accessories),
                    (&Route::GetPut { ref get, ref put }, Method::Put) => put.borrow_mut()
                        .handle(uri, body, controller_id, &database, &accessories),
                    _ => Box::new(future::ok(
                        Response::new().with_status(StatusCode::BadRequest)
                    )),
                }
            } else {
                Box::new(future::ok(Response::new().with_status(StatusCode::NotFound)))
            }
        }))
    }
}

pub fn serve<S: 'static + Storage + Send>(
    socket_addr: &SocketAddr,
    config: Arc<Config>,
    database: Arc<Mutex<Database<S>>>,
    accessories: AccessoryList,
) {
    let mut evt_loop = Core::new().unwrap();
    let listener = TcpListener::bind(socket_addr, &evt_loop.handle()).unwrap();
    let http: Http<hyper::Chunk> = Http::new();
    let handle = evt_loop.handle();

    let server = listener.incoming().for_each(|(stream, _)| {
        let (stream, sender) = EncryptedStream::new(stream);
        let controller_id = Arc::new(stream.controller_id);
        handle.spawn(http.serve_connection(stream, Api::new(
            controller_id.clone(),
            config.clone(),
            database.clone(),
            accessories.clone(),
            sender,
        )).map_err(|_| ()).map(|_| ()));
        Ok(())
    });
    evt_loop.run(server).unwrap();
}
