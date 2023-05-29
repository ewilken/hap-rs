use futures::future::{BoxFuture, FutureExt};
use hyper::{body::Buf, Body};
use log::{debug, info};
use std::{ops::Deref, str};
use uuid::Uuid;

use crate::{
    event::Event,
    pairing::{Pairing, Permissions},
    pointer,
    tlv::{self, Type, Value},
    transport::http::handler::TlvHandlerExt,
};

pub struct Pairings;

impl Pairings {
    pub fn new() -> Pairings { Pairings }
}

#[derive(Debug, Clone)]
enum StepNumber {
    Unknown = 0,
    Res = 2,
}

#[derive(Debug, Clone)]
enum HandlerNumber {
    Add = 3,
    Remove = 4,
    List = 5,
}

pub enum HandlerType {
    Add {
        pairing_id: Vec<u8>,
        ltpk: Vec<u8>,
        permissions: Permissions,
    },
    Remove {
        pairing_id: Vec<u8>,
    },
    List,
}

impl TlvHandlerExt for Pairings {
    type ParseResult = HandlerType;
    type Result = tlv::Container;

    fn parse(&self, body: Body) -> BoxFuture<Result<HandlerType, tlv::ErrorContainer>> {
        async {
            let aggregated_body = hyper::body::aggregate(body)
                .await
                .map_err(|_| tlv::ErrorContainer::new(StepNumber::Unknown as u8, tlv::Error::Unknown))?;

            debug!("received body: {:?}", aggregated_body.chunk());

            let mut decoded = tlv::decode(aggregated_body.chunk());
            if decoded.get(&(Type::State as u8)) != Some(&vec![1]) {
                return Err(tlv::ErrorContainer::new(0, tlv::Error::Unknown));
            }
            match decoded.get(&(Type::Method as u8)) {
                Some(handler) => match handler[0] {
                    x if x == HandlerNumber::Add as u8 => {
                        let pairing_id = decoded
                            .remove(&(Type::Identifier as u8))
                            .ok_or(tlv::ErrorContainer::new(StepNumber::Res as u8, tlv::Error::Unknown))?;
                        let ltpk = decoded
                            .remove(&(Type::PublicKey as u8))
                            .ok_or(tlv::ErrorContainer::new(StepNumber::Res as u8, tlv::Error::Unknown))?;
                        let perms = decoded
                            .remove(&(Type::Permissions as u8))
                            .ok_or(tlv::ErrorContainer::new(StepNumber::Res as u8, tlv::Error::Unknown))?;
                        let permissions = Permissions::from_byte(perms[0])
                            .map_err(|_| tlv::ErrorContainer::new(StepNumber::Res as u8, tlv::Error::Unknown))?;
                        Ok(HandlerType::Add {
                            pairing_id,
                            ltpk,
                            permissions,
                        })
                    },
                    x if x == HandlerNumber::Remove as u8 => {
                        let pairing_id = decoded
                            .remove(&(Type::Identifier as u8))
                            .ok_or(tlv::ErrorContainer::new(StepNumber::Res as u8, tlv::Error::Unknown))?;
                        Ok(HandlerType::Remove { pairing_id })
                    },
                    x if x == HandlerNumber::List as u8 => Ok(HandlerType::List),
                    _ => Err(tlv::ErrorContainer::new(StepNumber::Unknown as u8, tlv::Error::Unknown)),
                },
                None => Err(tlv::ErrorContainer::new(StepNumber::Unknown as u8, tlv::Error::Unknown)),
            }
        }
        .boxed()
    }

    fn handle(
        &mut self,
        handler: HandlerType,
        controller_id: pointer::ControllerId,
        config: pointer::Config,
        storage: pointer::Storage,
        event_emitter: pointer::EventEmitter,
    ) -> BoxFuture<Result<tlv::Container, tlv::ErrorContainer>> {
        async move {
            match handler {
                HandlerType::Add {
                    pairing_id,
                    ltpk,
                    permissions,
                } => match handle_add(
                    controller_id,
                    config,
                    storage,
                    event_emitter,
                    pairing_id,
                    ltpk,
                    permissions,
                )
                .await
                {
                    Ok(res) => Ok(res),
                    Err(err) => Err(tlv::ErrorContainer::new(StepNumber::Res as u8, err)),
                },
                HandlerType::Remove { pairing_id } => {
                    match handle_remove(controller_id, storage, event_emitter, pairing_id).await {
                        Ok(res) => Ok(res),
                        Err(err) => Err(tlv::ErrorContainer::new(StepNumber::Res as u8, err)),
                    }
                },
                HandlerType::List => match handle_list(controller_id, storage).await {
                    Ok(res) => Ok(res),
                    Err(err) => Err(tlv::ErrorContainer::new(StepNumber::Res as u8, err)),
                },
            }
        }
        .boxed()
    }
}

async fn handle_add(
    controller_id: pointer::ControllerId,
    config: pointer::Config,
    storage: pointer::Storage,
    event_emitter: pointer::EventEmitter,
    pairing_id: Vec<u8>,
    ltpk: Vec<u8>,
    permissions: Permissions,
) -> Result<tlv::Container, tlv::Error> {
    info!("pairings M1: received add pairing request");

    check_admin(&controller_id, &storage).await?;

    let uuid_str = str::from_utf8(&pairing_id)?;
    let pairing_uuid = Uuid::parse_str(uuid_str)?;

    let mut s = storage.lock().await;
    match s.load_pairing(&pairing_uuid).await {
        Ok(mut pairing) => {
            if x25519_dalek::PublicKey::from(pairing.public_key)
                != x25519_dalek::PublicKey::from(TryInto::<[u8; 32]>::try_into(ltpk).map_err(|_| tlv::Error::Unknown)?)
            {
                return Err(tlv::Error::Unknown);
            }
            pairing.permissions = permissions;
            s.save_pairing(&pairing).await?;

            drop(s);

            event_emitter
                .lock()
                .await
                .emit(&Event::ControllerPaired { id: pairing.id })
                .await;
        },
        Err(_) => {
            if let Some(max_peers) = config.lock().await.max_peers {
                if s.count_pairings().await? + 1 > max_peers {
                    return Err(tlv::Error::MaxPeers);
                }
            }

            let mut public_key = [0; 32];
            public_key.clone_from_slice(&ltpk);
            let pairing = Pairing {
                id: pairing_uuid,
                permissions,
                public_key,
            };
            s.save_pairing(&pairing).await?;

            drop(s);

            event_emitter
                .lock()
                .await
                .emit(&Event::ControllerPaired { id: pairing.id })
                .await;
        },
    }

    info!("pairings M2: sending add pairing response");

    Ok(vec![Value::State(StepNumber::Res as u8)])
}

async fn handle_remove(
    controller_id: pointer::ControllerId,
    storage: pointer::Storage,
    event_emitter: pointer::EventEmitter,
    pairing_id: Vec<u8>,
) -> Result<tlv::Container, tlv::Error> {
    info!("pairings M1: received remove pairing request");

    check_admin(&controller_id, &storage).await?;

    let uuid_str = str::from_utf8(&pairing_id)?;
    let pairing_uuid = Uuid::parse_str(uuid_str)?;
    storage.lock().await.delete_pairing(&pairing_uuid).await?;

    event_emitter
        .lock()
        .await
        .emit(&Event::ControllerUnpaired { id: pairing_uuid })
        .await;

    info!("pairings M2: sending remove pairing response");

    Ok(vec![Value::State(StepNumber::Res as u8)])
}

async fn handle_list(
    controller_id: pointer::ControllerId,
    storage: pointer::Storage,
) -> Result<tlv::Container, tlv::Error> {
    info!("pairings M1: received list pairings request");

    check_admin(&controller_id, &storage).await?;

    let pairings = storage.lock().await.list_pairings().await?;
    let mut list = vec![Value::State(StepNumber::Res as u8)];
    for (i, pairing) in pairings.iter().enumerate() {
        list.push(Value::Identifier(pairing.id.hyphenated().to_string()));
        list.push(Value::PublicKey(pairing.public_key.to_vec()));
        list.push(Value::Permissions(pairing.permissions.clone()));
        if i < pairings.len() {
            list.push(Value::Separator);
        }
    }

    info!("pairings M2: sending list pairings response");

    Ok(list)
}

async fn check_admin(controller_id: &pointer::ControllerId, storage: &pointer::Storage) -> Result<(), tlv::Error> {
    let controller_id: Uuid = controller_id
        .read()
        .unwrap()
        .deref()
        .ok_or(tlv::Error::Authentication)?;
    match storage.lock().await.load_pairing(&controller_id).await {
        Err(_) => Err(tlv::Error::Authentication),
        Ok(controller) => match controller.permissions {
            Permissions::Admin => Ok(()),
            _ => Err(tlv::Error::Authentication),
        },
    }
}
