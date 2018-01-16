use std::io::Error;
use rand::{Rng, OsRng};
use sha2::Sha512;
use ed25519_dalek::Keypair;

pub struct Entity {
    name: String,
    public_key: [u8; 32],
    private_key: [u8; 32],
}

impl Entity {
    fn new(name: String, public_key: [u8; 32], private_key: [u8; 32]) -> Entity {
        Entity {name, public_key, private_key}
    }

    fn new_random(name: String) -> Result<Entity, Error> {
        let keypair = generate_key_pair()?;
        Ok(Entity {
            name,
            public_key: keypair.public.to_bytes(),
            private_key: keypair.secret.to_bytes(),
        })
    }
}

fn generate_key_pair() -> Result<Keypair, Error> {
    let mut cspring = OsRng::new()?;
    Ok(Keypair::generate::<Sha512>(&mut cspring))
}
