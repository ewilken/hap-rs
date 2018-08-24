use std::{cell, io, str, num, sync::mpsc};

use serde_json;
use hyper;
use chacha20_poly1305_aead;
use eui48;

/// Error wrapper type.
#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "IO Error {}", _0)]
    Io(#[cause] io::Error),
    #[fail(display = "JSON Error {}", _0)]
    Json(#[cause] serde_json::Error),
    #[fail(display = "HTTP Status Code {}", _0)]
    HttpStatus(hyper::StatusCode),
    #[fail(display = "Borrow Error {}", _0)]
    Borrow(#[cause] cell::BorrowError),
    #[fail(display = "BorrowMut Error {}", _0)]
    BorrowMut(#[cause] cell::BorrowMutError),
    #[fail(display = "ChaCha20-Poly1305-AEAD Error {}", _0)]
    ChaCha20Poly1305Aead(#[cause] chacha20_poly1305_aead::DecryptError),
    #[fail(display = "UTF-8 Error {}", _0)]
    Utf8(#[cause] str::Utf8Error),
    #[fail(display = "MAC Address Parse Error {}", _0)]
    MacAddressParse(#[cause] eui48::ParseError),
    #[fail(display = "Parse Int Error {}", _0)]
    ParseInt(#[cause] num::ParseIntError),
    #[fail(display = "MPSC Send Error {}", _0)]
    MpscSend(#[cause] mpsc::SendError<()>),
}

impl Error {
    /// Creates a new `std::io::Error` wrapped in a `hap::Error::Io()`.
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

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Error {
        Error::Utf8(err)
    }
}

impl From<eui48::ParseError> for Error {
    fn from(err: eui48::ParseError) -> Error {
        Error::MacAddressParse(err)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Error {
        Error::ParseInt(err)
    }
}

impl From<mpsc::SendError<()>> for Error {
    fn from(err: mpsc::SendError<()>) -> Error {
        Error::MpscSend(err)
    }
}
