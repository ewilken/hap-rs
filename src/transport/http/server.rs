use std::{
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
};

use futures::{
    channel::oneshot,
    future::{self, BoxFuture, Future, FutureExt, TryFutureExt},
    lock::Mutex as FutureMutex,
    stream::StreamExt,
};
use hyper::{server::conn::Http, service::Service, Body, Method, Request, Response, StatusCode};
use log::{debug, error};
use tokio::net::TcpListener;

use crate::{
    event::Event,
    pointer,
    transport::{
        http::{
            event_response,
            handler::{
                accessories::Accessories,
                characteristics::{GetCharacteristics, UpdateCharacteristics},
                identify::Identify,
                pair_setup::PairSetup,
                pair_verify::PairVerify,
                pairings::Pairings,
                HandlerExt,
                JsonHandler,
                TlvHandler,
            },
            status_response,
            EventObject,
        },
        tcp::{EncryptedStream, Session, StreamWrapper},
    },
    Error,
    Result,
};

struct Handlers {
    pub pair_setup: Arc<FutureMutex<Box<dyn HandlerExt + Send + Sync>>>,
    pub pair_verify: Arc<FutureMutex<Box<dyn HandlerExt + Send + Sync>>>,
    pub accessories: Arc<FutureMutex<Box<dyn HandlerExt + Send + Sync>>>,
    pub get_characteristics: Arc<FutureMutex<Box<dyn HandlerExt + Send + Sync>>>,
    pub put_characteristics: Arc<FutureMutex<Box<dyn HandlerExt + Send + Sync>>>,
    pub pairings: Arc<FutureMutex<Box<dyn HandlerExt + Send + Sync>>>,
    pub identify: Arc<FutureMutex<Box<dyn HandlerExt + Send + Sync>>>,
}

struct Api {
    controller_id: pointer::ControllerId,
    event_subscriptions: pointer::EventSubscriptions,
    config: pointer::Config,
    storage: pointer::Storage,
    accessory_list: pointer::AccessoryList,
    event_emitter: pointer::EventEmitter,
    handlers: Handlers,
}

impl Api {
    fn new(
        controller_id: pointer::ControllerId,
        event_subscriptions: pointer::EventSubscriptions,
        config: pointer::Config,
        storage: pointer::Storage,
        accessory_list: pointer::AccessoryList,
        event_emitter: pointer::EventEmitter,
        session_sender: oneshot::Sender<Session>,
    ) -> Self {
        Api {
            controller_id,
            event_subscriptions,
            config,
            storage,
            accessory_list,
            event_emitter,
            handlers: Handlers {
                pair_setup: Arc::new(FutureMutex::new(Box::new(TlvHandler::from(PairSetup::new())))),
                pair_verify: Arc::new(FutureMutex::new(Box::new(TlvHandler::from(PairVerify::new(
                    session_sender,
                ))))),
                accessories: Arc::new(FutureMutex::new(Box::new(JsonHandler::from(Accessories::new())))),
                get_characteristics: Arc::new(FutureMutex::new(Box::new(JsonHandler::from(GetCharacteristics::new())))),
                put_characteristics: Arc::new(FutureMutex::new(Box::new(JsonHandler::from(
                    UpdateCharacteristics::new(),
                )))),
                pairings: Arc::new(FutureMutex::new(Box::new(TlvHandler::from(Pairings::new())))),
                identify: Arc::new(FutureMutex::new(Box::new(JsonHandler::from(Identify::new())))),
            },
        }
    }
}

impl Service<Request<Body>> for Api {
    // type Error = http::Error;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = std::result::Result<Self::Response, Self::Error>> + Send>>;
    type Response = Response<Body>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<std::result::Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let (parts, body) = req.into_parts();
        let method = parts.method;
        let uri = parts.uri;

        let mut handler: Option<Arc<FutureMutex<Box<dyn HandlerExt + Send + Sync>>>> = match (method, uri.path()) {
            (Method::POST, "/pair-setup") => Some(self.handlers.pair_setup.clone()),
            (Method::POST, "/pair-verify") => Some(self.handlers.pair_verify.clone()),
            (Method::GET, "/accessories") => Some(self.handlers.accessories.clone()),
            (Method::GET, "/characteristics") => Some(self.handlers.get_characteristics.clone()),
            (Method::PUT, "/characteristics") => Some(self.handlers.put_characteristics.clone()),
            (Method::POST, "/pairings") => Some(self.handlers.pairings.clone()),
            (Method::POST, "/identify") => Some(self.handlers.identify.clone()),
            _ => None,
        };

        let controller_id = self.controller_id.clone();
        let event_subscriptions = self.event_subscriptions.clone();
        let config = self.config.clone();
        let storage = self.storage.clone();
        let accessory_list = self.accessory_list.clone();
        let event_emitter = self.event_emitter.clone();

        let fut = async move {
            match handler.take() {
                Some(handler) =>
                    handler
                        .lock()
                        .await
                        .handle(
                            uri,
                            body,
                            controller_id,
                            event_subscriptions,
                            config,
                            storage,
                            accessory_list,
                            event_emitter,
                        )
                        .await,
                None => future::ready(status_response(StatusCode::NOT_FOUND)).await,
            }
        }
        .boxed();

        fut
    }
}

#[derive(Clone)]
pub struct Server {
    config: pointer::Config,
    storage: pointer::Storage,
    accessory_list: pointer::AccessoryList,
    event_emitter: pointer::EventEmitter,
}

impl Server {
    pub fn new(
        config: pointer::Config,
        storage: pointer::Storage,
        accessory_list: pointer::AccessoryList,
        event_emitter: pointer::EventEmitter,
    ) -> Self {
        Server {
            config,
            storage,
            accessory_list,
            event_emitter,
        }
    }

    pub fn run_handle(&self) -> BoxFuture<Result<()>> {
        let config = self.config.clone();
        let storage = self.storage.clone();
        let accessory_list = self.accessory_list.clone();
        let event_emitter = self.event_emitter.clone();

        async move {
            let socket_addr = config.lock().expect("accessing config").socket_addr;
            let mut listener = TcpListener::bind(socket_addr).await?;

            debug!("binding TCP listener on {}", &socket_addr);

            let mut incoming = listener.incoming();

            while let Some(stream) = incoming.next().await {
                let stream = stream?;

                debug!("incoming TCP stream from {}", stream.peer_addr()?);

                let (encrypted_stream, stream_incoming, stream_outgoing, session_sender) = EncryptedStream::new(stream);
                let stream_wrapper = StreamWrapper::new(stream_incoming, stream_outgoing.clone());
                let event_subscriptions = Arc::new(Mutex::new(vec![]));

                let api = Api::new(
                    encrypted_stream.controller_id.clone(),
                    event_subscriptions.clone(),
                    config.clone(),
                    storage.clone(),
                    accessory_list.clone(),
                    event_emitter.clone(),
                    session_sender,
                );

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
                                    let event_res =
                                        event_response(vec![event]).expect("couldn't create event response");
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

                let http = Http::new();

                // future::join(encrypted_stream, http.serve_connection(stream_wrapper, api)).await;

                // encrypted_stream
                //     .map_err(|e| error!("{}", e))
                //     .join(http.serve_connection(stream_wrapper, api).map_err(|e| error!("{}", e)))
                //     .map(|_| ())
                //     .then(|_| Ok(()))

                future::join(
                    encrypted_stream.map_err(|e| error!("{:?}", e)).map(|_| ()),
                    http.serve_connection(stream_wrapper, api)
                        .map_err(|e| error!("{:?}", e))
                        .map(|_| ()),
                )
                .await;
            }

            Ok(())
        }
        .boxed()
    }
}
