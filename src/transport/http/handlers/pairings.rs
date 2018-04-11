use std::str;

use uuid::Uuid;

use db::database::DatabasePtr;
use config::ConfigPtr;
use transport::{http::handlers::TlvHandler, tlv::{self, Type, Value}};
use protocol::pairing::{Pairing, Permissions};

pub struct Pairings {}

impl Pairings {
    pub fn new() -> Pairings {
        Pairings {}
    }
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
                // TODO - put those handler numbers into the handler type enum somehow
                3 => {
                    let pairing_id = decoded.get(&(Type::Identifier as u8)).ok_or(
                        tlv::ErrorContainer::new(2, tlv::Error::Unknown)
                    )?;
                    let ltpk = decoded.get(&(Type::PublicKey as u8)).ok_or(
                        tlv::ErrorContainer::new(2, tlv::Error::Unknown)
                    )?;
                    let perms = decoded.get(&(Type::Permissions as u8)).ok_or(
                        tlv::ErrorContainer::new(2, tlv::Error::Unknown)
                    )?;
                    let permissions = Permissions::from_u8(perms[0]).map_err(
                        |_| tlv::ErrorContainer::new(2, tlv::Error::Unknown)
                    )?;
                    Ok(HandlerType::Add {
                        pairing_id: pairing_id.clone(),
                        ltpk: ltpk.clone(),
                        permissions,
                    })
                },
                4 => {
                    let pairing_id = decoded.get(&(Type::Identifier as u8)).ok_or(
                        tlv::ErrorContainer::new(2, tlv::Error::Unknown)
                    )?;
                    Ok(HandlerType::Remove { pairing_id: pairing_id.clone() })
                },
                5 => {
                    Ok(HandlerType::List)
                },
                _ => Err(tlv::ErrorContainer::new(0, tlv::Error::Unknown))
            },
            None => Err(tlv::ErrorContainer::new(0, tlv::Error::Unknown)),
        }
    }

    fn handle(
        &mut self,
        handler: HandlerType,
        config: &ConfigPtr,
        database: &DatabasePtr,
    ) -> Result<tlv::Container, tlv::ErrorContainer> {
        match handler {
            HandlerType::Add { pairing_id, ltpk, permissions } => match handle_add(
                config,
                database,
                pairing_id,
                ltpk,
                permissions
            ) {
                Ok(res) => Ok(res),
                Err(err) => Err(tlv::ErrorContainer::new(2, err)),
            },
            HandlerType::Remove { pairing_id } => match handle_remove(database, pairing_id) {
                Ok(res) => Ok(res),
                Err(err) => Err(tlv::ErrorContainer::new(2, err)),
            },
            HandlerType::List => match handle_list(database) {
                Ok(res) => Ok(res),
                Err(err) => Err(tlv::ErrorContainer::new(2, err)),
            },
        }
    }
}

fn handle_add(
    config: &ConfigPtr,
    database: &DatabasePtr,
    pairing_id: Vec<u8>,
    ltpk: Vec<u8>,
    permissions: Permissions,
) -> Result<tlv::Container, tlv::Error> {
    // TODO - check if controller is admin

    let uuid_str = str::from_utf8(&pairing_id)?;
    let pairing_uuid = Uuid::parse_str(uuid_str)?;

    let d = database.lock().unwrap();
    match d.get_pairing(pairing_uuid) {
        Ok(mut pairing) => {
            if &pairing.public_key.to_vec() != &ltpk {
                return Err(tlv::Error::Unknown);
            }
            pairing.permissions = permissions;
            d.set_pairing(&pairing)?;
        },
        Err(_) => {
            if let Some(max_peers) = config.max_peers {
                let count = d.count_pairings()?;
                if count + 1 > max_peers {
                    return Err(tlv::Error::MaxPeers);
                }
            }

            let mut public_key = [0; 32];
            public_key.clone_from_slice(&ltpk);
            let pairing = Pairing {id: pairing_uuid, permissions, public_key};
            d.set_pairing(&pairing)?;
        },
    }

    Ok(vec![Value::State(2)])
}

fn handle_remove(
    database: &DatabasePtr,
    pairing_id: Vec<u8>,
) -> Result<tlv::Container, tlv::Error> {
    // TODO - check if controller is admin

    let uuid_str = str::from_utf8(&pairing_id)?;
    let pairing_uuid = Uuid::parse_str(uuid_str)?;
    let d = database.lock().unwrap();
    d.get_pairing(pairing_uuid).map(|pairing| d.delete_pairing(&pairing.id))?;

    Ok(vec![Value::State(2)])
}

fn handle_list(
    database: &DatabasePtr,
) -> Result<tlv::Container, tlv::Error> {
    // TODO - check if controller is admin

    let d = database.lock().unwrap();
    let pairings = d.list_pairings()?;
    let mut list = vec![Value::State(2)];
    for (i, pairing) in pairings.iter().enumerate() {
        list.push(Value::Identifier(pairing.id.hyphenated().to_string()));
        list.push(Value::PublicKey(pairing.public_key.to_vec()));
        list.push(Value::Permissions(pairing.permissions.clone()));
        if i < pairings.len() {
            list.push(Value::Separator);
        }
    }

    Ok(list)
}
