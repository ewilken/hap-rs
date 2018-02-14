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

use transport::http::handlers::{self, pair_setup, pair_verify, /*accessories, characteristics, pairings, identify*/};
use transport::http::encrypted_stream::EncryptedStream;
use db::storage::Storage;
use db::database::Database;
use config::Config;

enum Route<S: Storage> {
    Get(Box<RefCell<handlers::Handler<S>>>),
    Post(Box<RefCell<handlers::Handler<S>>>),
    Put(Box<RefCell<handlers::Handler<S>>>),
}

struct Api<S: Storage> {
    config: Arc<Config>,
    database: Arc<Mutex<Database<S>>>,
    router: Arc<Router<Route<S>>>,
}

impl<S: Storage> Api<S> {
    fn new(config: Arc<Config>, database: Arc<Mutex<Database<S>>>, secret_sender: oneshot::Sender<[u8; 32]>) -> Api<S> {
        let mut router = Router::new();
        router.add("/pair-setup", Route::Post(Box::new(RefCell::new(pair_setup::PairSetup::new()))));
        router.add("/pair-verify", Route::Post(Box::new(RefCell::new(pair_verify::PairVerify::new(secret_sender)))));
        //router.add("/accessories", );
        //router.add("/characteristics", );
        //router.add("/characteristics", );
        //router.add("/pairings", );
        //router.add("/identify", );

        Api { config, database, router: Arc::new(router) }
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
        let database = self.database.clone();

        Box::new(body.fold(vec![], |mut v, c| {
            v.extend(c.to_vec());
            future::ok::<Vec<u8>, hyper::Error>(v)
        }).and_then(move |body| {
            if let Ok(route_match) = router.recognize(uri.path()) {
                match (route_match.handler, method) {
                    (&Route::Get(ref handler), Method::Get) => handler.borrow_mut()
                        .handle(uri, body, &database),
                    (&Route::Post(ref handler), Method::Post) => handler.borrow_mut()
                        .handle(uri, body, &database),
                    (&Route::Put(ref handler), Method::Put) => handler.borrow_mut()
                        .handle(uri, body, &database),
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

pub fn serve<S: 'static + Storage + Send>(socket_addr: &SocketAddr, config: Arc<Config>, database: Arc<Mutex<Database<S>>>) {
    let mut evt_loop = Core::new().unwrap();
    let listener = TcpListener::bind(socket_addr, &evt_loop.handle()).unwrap();
    let http: Http<hyper::Chunk> = Http::new();
    let handle = evt_loop.handle();

    let server = listener.incoming().for_each(|(stream, _)| {
        let (stream, sender) = EncryptedStream::new(stream);
        handle.spawn(http.serve_connection(stream, Api::new(config.clone(), database.clone(), sender)).map_err(|_| ()).map(|_| ()));
        Ok(())
    });
    evt_loop.run(server).unwrap();
}
