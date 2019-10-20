use std::{fmt, io, num, str, sync::mpsc};

use chacha20_poly1305_aead;
use eui48;
use failure::{self, err_msg, Context, Fail};
use hyper::{self, http};

/// ErrorKind wrapper type.
#[derive(Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "IO Error {}", _0)]
    Io(#[cause] io::Error),
    #[fail(display = "JSON Error {}", _0)]
    Json(#[cause] serde_json::Error),
    #[fail(display = "HTTP Status Code {}", _0)]
    HttpStatus(hyper::StatusCode),
    #[fail(display = "HTTP Error {}", _0)]
    Http(#[cause] http::Error),
    #[fail(display = "Hyper Error {}", _0)]
    Hyper(#[cause] hyper::error::Error),
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
    #[fail(display = "Error {}", _0)]
    Other(failure::Error),
}

#[derive(Debug)]
pub struct Error {
    kind: Context<ErrorKind>,
}

impl Error {
    /// Creates a new `Error` from a given `ErrorKind`.
    pub fn new(kind: ErrorKind) -> Error {
        Error {
            kind: Context::new(kind),
        }
    }

    /// Returns a reference to the `ErrorKind` of the `Error`.
    pub fn kind(&self) -> &ErrorKind { &self.kind.get_context() }

    /// Creates a new `Error` from a `&'static str`.
    pub fn from_str(cause: &'static str) -> Error { ErrorKind::Other(err_msg(cause)).into() }
}

// impl Fail for Error {
//     fn cause(&self) -> Option<&Fail> { self.kind.cause() }
//
//     fn backtrace(&self) -> Option<&Backtrace> { self.kind.backtrace() }
// }

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::Display::fmt(&self.kind, f) }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            kind: Context::new(kind),
        }
    }
}

impl From<failure::Error> for Error {
    fn from(err: failure::Error) -> Error { ErrorKind::Other(err).into() }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error { ErrorKind::Io(err).into() }
}

// TODO - fix it
// impl Into<io::Error> for Error {
//     fn into(self) -> io::Error {
//         io::Error::new(io::ErrorKind::Other, self.cause().into())
//     }
// }

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error { ErrorKind::Json(err).into() }
}

impl From<http::Error> for Error {
    fn from(err: http::Error) -> Error { ErrorKind::Http(err).into() }
}

impl From<hyper::error::Error> for Error {
    fn from(err: hyper::error::Error) -> Error { ErrorKind::Hyper(err).into() }
}

impl From<chacha20_poly1305_aead::DecryptError> for Error {
    fn from(err: chacha20_poly1305_aead::DecryptError) -> Error { ErrorKind::ChaCha20Poly1305Aead(err).into() }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Error { ErrorKind::Utf8(err).into() }
}

impl From<eui48::ParseError> for Error {
    fn from(err: eui48::ParseError) -> Error { ErrorKind::MacAddressParse(err).into() }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Error { ErrorKind::ParseInt(err).into() }
}

impl From<mpsc::SendError<()>> for Error {
    fn from(err: mpsc::SendError<()>) -> Error { ErrorKind::MpscSend(err).into() }
}
