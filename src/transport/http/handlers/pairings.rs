use std::str;

use uuid::Uuid;

use db::DatabasePtr;
use config::ConfigPtr;
use transport::http::handlers::TlvHandler;
use protocol::{Pairing, Permissions, tlv::{self, Type, Value}, IdPtr};
use event::{Event, EmitterPtr};

pub struct Pairings {}

impl Pairings {
    pub fn new() -> Pairings {
        Pairings {}
    }
}

enum StepNumber {
    Unknown = 0,
    Res = 2,
}

enum HandlerNumber {
    Add = 3,
    Remove = 4,
    List = 5,
}

pub enum HandlerType {
    Add { pairing_id: Vec<u8>, ltpk: Vec<u8>, permissions: Permissions },
    Remove { pairing_id: Vec<u8> },
    List,
}

impl TlvHandler for Pairings {
    type ParseResult = HandlerType;
    type Result = tlv::Container;

    fn parse(&self, body: Vec<u8>) -> Result<HandlerType, tlv::ErrorContainer> {
        let decoded = tlv::decode(body);
        if decoded.get(&(Type::State as u8)) != Some(&vec![1]) {
            return Err(tlv::ErrorContainer::new(0, tlv::Error::Unknown));
        }
        match decoded.get(&(Type::Method as u8)) {
            Some(handler) => match handler[0] {
                x if x == HandlerNumber::Add as u8 => {
                    let pairing_id = decoded.get(&(Type::Identifier as u8)).ok_or(
                        tlv::ErrorContainer::new(StepNumber::Res as u8, tlv::Error::Unknown)
                    )?;
                    let ltpk = decoded.get(&(Type::PublicKey as u8)).ok_or(
                        tlv::ErrorContainer::new(StepNumber::Res as u8, tlv::Error::Unknown)
                    )?;
                    let perms = decoded.get(&(Type::Permissions as u8)).ok_or(
                        tlv::ErrorContainer::new(StepNumber::Res as u8, tlv::Error::Unknown)
                    )?;
                    let permissions = Permissions::from_u8(perms[0]).map_err(
                        |_| tlv::ErrorContainer::new(StepNumber::Res as u8, tlv::Error::Unknown)
                    )?;
                    Ok(HandlerType::Add {
                        pairing_id: pairing_id.clone(),
                        ltpk: ltpk.clone(),
                        permissions,
                    })
                },
                x if x == HandlerNumber::Remove as u8 => {
                    let pairing_id = decoded.get(&(Type::Identifier as u8)).ok_or(
                        tlv::ErrorContainer::new(StepNumber::Res as u8, tlv::Error::Unknown)
                    )?;
                    Ok(HandlerType::Remove { pairing_id: pairing_id.clone() })
                },
                x if x == HandlerNumber::List as u8 => {
                    Ok(HandlerType::List)
                },
                _ => Err(tlv::ErrorContainer::new(StepNumber::Unknown as u8, tlv::Error::Unknown))
            },
            None => Err(tlv::ErrorContainer::new(StepNumber::Unknown as u8, tlv::Error::Unknown)),
        }
    }

    fn handle(
        &mut self,
        handler: HandlerType,
        controller_id: &IdPtr,
        config: &ConfigPtr,
        database: &DatabasePtr,
        event_emitter: &EmitterPtr,
    ) -> Result<tlv::Container, tlv::ErrorContainer> {
        match handler {
            HandlerType::Add { pairing_id, ltpk, permissions } => match handle_add(
                config,
                database,
                event_emitter,
                controller_id,
                pairing_id,
                ltpk,
                permissions,
            ) {
                Ok(res) => Ok(res),
                Err(err) => Err(tlv::ErrorContainer::new(StepNumber::Res as u8, err)),
            },
            HandlerType::Remove { pairing_id } => match handle_remove(
                database,
                event_emitter,
                controller_id,
                pairing_id,
            ) {
                Ok(res) => Ok(res),
                Err(err) => Err(tlv::ErrorContainer::new(StepNumber::Res as u8, err)),
            },
            HandlerType::List => match handle_list(database, controller_id) {
                Ok(res) => Ok(res),
                Err(err) => Err(tlv::ErrorContainer::new(StepNumber::Res as u8, err)),
            },
        }
    }
}

fn handle_add(
    config: &ConfigPtr,
    database: &DatabasePtr,
    event_emitter: &EmitterPtr,
    controller_id: &IdPtr,
    pairing_id: Vec<u8>,
    ltpk: Vec<u8>,
    permissions: Permissions,
) -> Result<tlv::Container, tlv::Error> {
    println!("/pairings - M1: Got Add Pairing Request");

    check_admin(database, controller_id)?;

    let uuid_str = str::from_utf8(&pairing_id)?;
    let pairing_uuid = Uuid::parse_str(uuid_str)?;

    let d = database.try_borrow_mut()?;
    match d.get_pairing(pairing_uuid) {
        Ok(mut pairing) => {
            if &pairing.public_key.to_vec() != &ltpk {
                return Err(tlv::Error::Unknown);
            }
            pairing.permissions = permissions;
            d.set_pairing(&pairing)?;
            drop(d);

            event_emitter.try_borrow()?.emit(Event::DevicePaired);
        },
        Err(_) => {
            if let Some(max_peers) = config.try_borrow()?.max_peers {
                if d.count_pairings()? + 1 > max_peers {
                    return Err(tlv::Error::MaxPeers);
                }
            }

            let mut public_key = [0; 32];
            public_key.clone_from_slice(&ltpk);
            let pairing = Pairing {id: pairing_uuid, permissions, public_key};
            d.set_pairing(&pairing)?;
            drop(d);

            event_emitter.try_borrow()?.emit(Event::DevicePaired);
        },
    }

    println!("/pairings - M2: Sending Add Pairing Response");

    Ok(vec![Value::State(StepNumber::Res as u8)])
}

fn handle_remove(
    database: &DatabasePtr,
    event_emitter: &EmitterPtr,
    controller_id: &IdPtr,
    pairing_id: Vec<u8>,
) -> Result<tlv::Container, tlv::Error> {
    println!("/pairings - M1: Got Remove Pairing Request");

    check_admin(database, controller_id)?;

    let uuid_str = str::from_utf8(&pairing_id)?;
    let pairing_uuid = Uuid::parse_str(uuid_str)?;
    let d = database.try_borrow_mut()?;
    d.delete_pairing(&d.get_pairing(pairing_uuid)?.id)?;
    drop(d);

    event_emitter.try_borrow()?.emit(Event::DeviceUnpaired);

    println!("/pairings - M2: Sending Remove Pairing Response");

    Ok(vec![Value::State(StepNumber::Res as u8)])
}

fn handle_list(
    database: &DatabasePtr,
    controller_id: &IdPtr,
) -> Result<tlv::Container, tlv::Error> {
    println!("/pairings - M1: Got List Pairings Request");

    check_admin(database, controller_id)?;

    let pairings = database.try_borrow()?.list_pairings()?;
    let mut list = vec![Value::State(StepNumber::Res as u8)];
    for (i, pairing) in pairings.iter().enumerate() {
        list.push(Value::Identifier(pairing.id.hyphenated().to_string()));
        list.push(Value::PublicKey(pairing.public_key.to_vec()));
        list.push(Value::Permissions(pairing.permissions.clone()));
        if i < pairings.len() {
            list.push(Value::Separator);
        }
    }

    println!("/pairings - M2: Sending List Pairings Response");

    Ok(list)
}

fn check_admin(database: &DatabasePtr, controller_id: &IdPtr) -> Result<(), tlv::Error> {
    let err = tlv::Error::Authentication;
    match database.try_borrow()?.get_pairing(controller_id.try_borrow()?.ok_or(err)?) {
        Err(_) => Err(err),
        Ok(controller) => match controller.permissions {
            Permissions::Admin => Ok(()),
            _ => Err(err),
        },
    }
}
