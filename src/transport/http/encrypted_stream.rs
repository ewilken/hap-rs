use std::io::{self, Read, Write};
use futures::{Async, Future, Poll};
use futures::sync::oneshot;
use tokio_core::net::TcpStream;
use tokio_io::{AsyncRead, AsyncWrite};

pub struct EncryptedStream {
    stream: TcpStream,
    decrypt_receive: oneshot::Receiver<[u8; 32]>,
    session_key: Option<[u8; 32]>,
}

impl EncryptedStream {
    pub fn new(stream: TcpStream) -> (EncryptedStream, oneshot::Sender<[u8; 32]>) {
        let (sender, receiver) = oneshot::channel();
        (EncryptedStream { stream, decrypt_receive: receiver, session_key: None }, sender)
    }
}

impl Read for EncryptedStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        if self.session_key.is_none() {
            match self.decrypt_receive.poll() {
                Ok(Async::Ready(session_key)) => {
                    self.session_key = Some(session_key);
                },
                _ => {},
            }
        }
        self.stream.read(buf)
    }
}

impl Write for EncryptedStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        self.stream.write(buf)
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        self.stream.flush()
    }
}

impl AsyncRead for EncryptedStream {}

impl AsyncWrite for EncryptedStream {
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        AsyncWrite::shutdown(&mut self.stream)
    }
}
