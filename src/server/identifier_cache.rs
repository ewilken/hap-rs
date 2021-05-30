use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::HapType;

/// `IdentifierCache` persists the `aid`s & `iid`s for the accessories on the server.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IdentifierCache {
    pub aid_cache: HashMap<Uuid, u8>,
    pub iid_cache: HashMap<(Uuid, Uuid, Option<HapType>, Option<Uuid>), u8>,
    pub next_aid: u8,
    pub next_iids: HashMap<Uuid, u8>,
}

impl IdentifierCache {
    pub fn new() -> Self {
        IdentifierCache {
            aid_cache: HashMap::new(),
            iid_cache: HashMap::new(),
            next_aid: 2, // 1 is the root Accessory or Bridge
            next_iids: HashMap::new(),
        }
    }

    pub fn get_aid(&mut self, accessory_id: Uuid) -> u8 {
        match self.aid_cache.get(&accessory_id) {
            Some(aid) => *aid,
            None => {
                let aid = self.get_next_aid();
                self.aid_cache.insert(accessory_id, aid);
                aid
            },
        }
    }

    pub fn get_iid(
        &mut self,
        accessory_id: Uuid,
        service_id: Uuid,
        service_subtype: Option<HapType>,
        characteristic_id: Option<Uuid>,
    ) -> u8 {
        match self
            .iid_cache
            .get(&(accessory_id, service_id, service_subtype, characteristic_id))
        {
            Some(iid) => *iid,
            None => {
                let iid = self.get_next_iid(accessory_id);
                self.iid_cache
                    .insert((accessory_id, service_id, service_subtype, characteristic_id), iid);
                iid
            },
        }
    }

    pub fn get_next_aid(&mut self) -> u8 {
        let next_aid = self.next_aid;
        self.next_aid += 1;
        next_aid
    }

    pub fn get_next_iid(&mut self, accessory_id: Uuid) -> u8 {
        match self.next_iids.get_mut(&accessory_id) {
            Some(iid) => {
                let current_iid = *iid;
                *iid += 1;
                current_iid
            },
            None => {
                let iid = 2; // 1 is the the Accessory Information Service
                self.next_iids.insert(accessory_id, iid);
                iid
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incrementing_ids() {
        let mut identifier_cache = IdentifierCache::new();

        let accessory_id_1 = Uuid::parse_str("e593c873-97eb-4aea-ad96-4492bc98a3a6").unwrap();
        let accessory_id_2 = Uuid::parse_str("78ec3702-e615-47b5-b5bc-95f1b92a5d78").unwrap();

        // it should start counting at 2
        assert_eq!(identifier_cache.get_aid(accessory_id_1), 2);

        // it should always return the same aid
        assert_eq!(identifier_cache.get_aid(accessory_id_1), 2);

        // it should increment the aid for the next accessory
        assert_eq!(identifier_cache.get_aid(accessory_id_2), 3);
    }

    #[test]
    fn test_json_serialization() {
        let mut identifier_cache = IdentifierCache::new();

        let accessory_id = Uuid::parse_str("e593c873-97eb-4aea-ad96-4492bc98a3a6").unwrap();
        let service_id = Uuid::parse_str("1b101b3b-a8bb-4a82-be0b-40086e532d28").unwrap();

        assert_eq!(identifier_cache.get_aid(accessory_id), 2);
        assert_eq!(identifier_cache.get_iid(accessory_id, service_id, None, None), 2);

        let json = serde_json::to_string(&identifier_cache).unwrap();
        dbg!(&json);

        assert_eq!(json, "{}".to_string());
    }

    #[test]
    fn test_json_deserialization() {}
}
