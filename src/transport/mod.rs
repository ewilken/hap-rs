use ring::hkdf;

pub(crate) mod bonjour;
pub(crate) mod http;
pub(crate) mod mdns;
pub(crate) mod tcp;

pub(crate) fn hkdf_extract_and_expand(salt: &[u8], secret: &[u8], data: &[u8]) -> [u8; 32] {
    let okm: WrappedBuffer = hkdf::Salt::new(hkdf::HKDF_SHA512, salt)
        .extract(secret)
        .expand(&[data], hkdf::HKDF_SHA512)
        .unwrap()
        .into();
    okm.0    
}

/// Generic newtype wrapper that lets us implement traits for externally-defined
/// types.
#[derive(Debug, PartialEq)]
struct WrappedBuffer([u8; 32]);

impl From<hkdf::Okm<'_, hkdf::Algorithm>> for WrappedBuffer {
    fn from(okm: hkdf::Okm<hkdf::Algorithm>) -> Self {
        let mut r = [0u8; 32];
        okm.fill(&mut r).unwrap();
        WrappedBuffer(r)
    }
}
