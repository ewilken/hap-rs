use std::{cell, io};

use serde_json;
use chacha20_poly1305_aead;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "IO Error {}", _0)]
    Io(#[cause] io::Error),
    #[fail(display = "JSON Error {}", _0)]
    Json(#[cause] serde_json::Error),
    #[fail(display = "Borrow Error {}", _0)]
    Borrow(#[cause] cell::BorrowError),
    #[fail(display = "BorrowMut Error {}", _0)]
    BorrowMut(#[cause] cell::BorrowMutError),
    #[fail(display = "ChaCha20-Poly1305-AEAD Error {}", _0)]
    ChaCha20Poly1305Aead(#[cause] chacha20_poly1305_aead::DecryptError),
}

impl Error {
    pub fn new_io(cause: &'static str) -> Error {
        Error::Io(io::Error::new(io::ErrorKind::Other, cause.to_owned()))
    }
}

impl From<Error> for io::Error {
    fn from(err: Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, format!("{:?}", err))
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
    }
}

impl From<cell::BorrowError> for Error {
    fn from(err: cell::BorrowError) -> Error {
        Error::Borrow(err)
    }
}

impl From<cell::BorrowMutError> for Error {
    fn from(err: cell::BorrowMutError) -> Error {
        Error::BorrowMut(err)
    }
}

impl From<chacha20_poly1305_aead::DecryptError> for Error {
    fn from(err: chacha20_poly1305_aead::DecryptError) -> Error {
        Error::ChaCha20Poly1305Aead(err)
    }
}
