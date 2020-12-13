use hkdf::Hkdf;
use sha2::Sha512;

use crate::{Error, Result};

pub(crate) mod bonjour;
pub(crate) mod http;
pub(crate) mod mdns;
pub(crate) mod tcp;

pub(crate) fn hkdf_extract_and_expand(salt: &[u8], ikm: &[u8], info: &[u8]) -> Result<[u8; 32]> {
    let mut okm = [0u8; 32];

    Hkdf::<Sha512>::new(Some(salt), ikm)
        .expand(info, &mut okm)
        .or(Err(Error::HkdfInvalidLength))?;

    Ok(okm)
}
