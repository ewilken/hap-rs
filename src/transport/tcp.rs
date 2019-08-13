use std::{
    cmp::min,
    io::{self, ErrorKind, Read, Write},
    sync::{Arc, Mutex},
};

use byteorder::{ByteOrder, LittleEndian};
use bytes::{buf::FromBuf, BytesMut};
use chacha20_poly1305_aead;
use futures::{
    sync::{
        mpsc::{self, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
    try_ready,
    Async::{self, NotReady, Ready},
    Future,
    Poll,
    Sink,
    Stream,
};
use ring::{digest, hkdf, hmac};
use tokio::{
    io::{AsyncRead, AsyncWrite},
    net::TcpStream,
};
use uuid::Uuid;

use crate::{protocol::IdPtr, Result};

pub struct StreamWrapper {
    incoming_receiver: UnboundedReceiver<Vec<u8>>,
    outgoing_sender: UnboundedSender<Vec<u8>>,
    incoming_buf: BytesMut,
}

impl StreamWrapper {
    pub fn new(
        incoming_receiver: UnboundedReceiver<Vec<u8>>,
        outgoing_sender: UnboundedSender<Vec<u8>>,
    ) -> StreamWrapper {
        StreamWrapper {
            incoming_receiver,
            outgoing_sender,
            incoming_buf: BytesMut::new(),
        }
    }

    fn poll_receiver(&mut self) -> std::result::Result<usize, io::Error> {
        match self.incoming_receiver.poll() {
            Ok(NotReady) => Err(ErrorKind::WouldBlock.into()),
            Ok(Ready(Some(incoming))) => {
                self.incoming_buf.extend_from_slice(&incoming);
                Ok(incoming.len())
            },
            Ok(Ready(None)) => Ok(0),
            // Err(_) => Err(Error::from_str("couldn't poll receiver").into()),
            Err(_) => Err(io::Error::new(io::ErrorKind::Other, "couldn't poll receiver")),
        }
    }
}

impl Read for StreamWrapper {
    fn read(&mut self, buf: &mut [u8]) -> std::result::Result<usize, io::Error> {
        self.poll_receiver()?;
        let r_len = min(buf.len(), self.incoming_buf.len());
        buf[..r_len].copy_from_slice(&self.incoming_buf[..r_len]);
        self.incoming_buf.advance(r_len);
        Ok(r_len)
    }
}

impl Write for StreamWrapper {
    fn write(&mut self, buf: &[u8]) -> std::result::Result<usize, io::Error> {
        self.outgoing_sender.unbounded_send(buf.to_vec())
            // .map_err(|_| Error::from_str("couldn't write").into())?;
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "couldn't write"))?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::result::Result<(), io::Error> {
        self.outgoing_sender.poll_complete()
            .map(|_| ())
            // .map_err(|_| Error::from_str("couldn't flush").into())
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "couldn't flush"))
    }
}

impl AsyncRead for StreamWrapper {}

impl AsyncWrite for StreamWrapper {
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        // TODO - maybe do something useful here
        Ok(Ready(()))
    }
}

pub struct Session {
    pub controller_id: Uuid,
    pub shared_secret: [u8; 32],
}

pub struct EncryptedStream {
    stream: TcpStream,
    incoming_sender: UnboundedSender<Vec<u8>>,
    outgoing_receiver: UnboundedReceiver<Vec<u8>>,
    session_receiver: oneshot::Receiver<Session>,
    pub controller_id: IdPtr,
    shared_secret: Option<[u8; 32]>,
    decrypt_count: u64,
    encrypt_count: u64,
    encrypted_buf: BytesMut,
    decrypted_buf: BytesMut,
    packet_len: usize,
    already_copied: usize,
    already_read: usize,
    decrypted_ready: bool,
    missing_data_for_decrypted_buf: bool,
    missing_data_for_encrypted_buf: bool,
}

impl EncryptedStream {
    pub fn new(
        stream: TcpStream,
    ) -> (
        EncryptedStream,
        UnboundedReceiver<Vec<u8>>,
        UnboundedSender<Vec<u8>>,
        oneshot::Sender<Session>,
    ) {
        let (sender, receiver) = oneshot::channel();
        let (incoming_sender, incoming_receiver) = mpsc::unbounded();
        let (outgoing_sender, outgoing_receiver) = mpsc::unbounded();
        (
            EncryptedStream {
                stream,
                incoming_sender,
                outgoing_receiver,
                session_receiver: receiver,
                controller_id: Arc::new(Mutex::new(None)),
                shared_secret: None,
                decrypt_count: 0,
                encrypt_count: 0,
                encrypted_buf: BytesMut::from_buf(vec![0; 1042]),
                decrypted_buf: BytesMut::from_buf(vec![0; 1024]),
                packet_len: 0,
                already_copied: 0,
                already_read: 0,
                decrypted_ready: false,
                missing_data_for_decrypted_buf: false,
                missing_data_for_encrypted_buf: false,
            },
            incoming_receiver,
            outgoing_sender,
            sender,
        )
    }

    fn read_decrypted(&mut self, buf: &mut [u8]) -> std::result::Result<usize, io::Error> {
        if self.decrypted_ready {
            let len = min(buf.len(), self.packet_len - 16);
            buf[..len].copy_from_slice(&self.decrypted_buf[..len]);
            self.already_copied = len;
            if self.already_copied == (self.packet_len - 16) {
                self.already_copied = 0;
                self.decrypted_ready = false;
            }

            return Ok(len);
        }

        Err(ErrorKind::WouldBlock.into())
    }

    fn read_encrypted(&mut self, buf: &mut [u8]) -> std::result::Result<usize, io::Error> {
        if self.missing_data_for_decrypted_buf {
            let decrypted = decrypt_chunk(
                &self.shared_secret.expect("missing shared secret"),
                &self.encrypted_buf[..2],
                &self.encrypted_buf[2..(self.packet_len - 14)],
                &self.encrypted_buf[(self.packet_len - 14)..(self.packet_len + 2)],
                &mut self.decrypt_count,
            )
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "decryption failed"))?;
            self.decrypted_buf[..decrypted.len()].copy_from_slice(&decrypted);
            self.missing_data_for_decrypted_buf = false;
            self.decrypted_ready = true;

            return self.read_decrypted(buf);
        }

        Err(ErrorKind::WouldBlock.into())
    }

    fn read_stream(&mut self, buf: &mut [u8]) -> std::result::Result<usize, io::Error> {
        if self.missing_data_for_encrypted_buf {
            let r_len = self.stream.read(&mut self.encrypted_buf[self.already_read..])?;

            if self.already_read + r_len == self.packet_len {
                self.already_read = 0;
                self.missing_data_for_encrypted_buf = false;
                self.missing_data_for_decrypted_buf = true;
                return self.read_encrypted(buf);
            }
            Err(ErrorKind::WouldBlock.into())
        } else {
            let r_len = self.stream.read(&mut self.encrypted_buf[self.already_read..2])?;
            self.already_read += r_len;

            if self.already_read == 2 {
                self.packet_len = LittleEndian::read_u16(&self.encrypted_buf) as usize + 16;

                self.missing_data_for_encrypted_buf = true;
                let r_len = self.stream.read(&mut self.encrypted_buf[self.already_read..])?;
                if r_len == self.packet_len {
                    self.already_read = 0;
                    self.missing_data_for_encrypted_buf = false;
                    self.missing_data_for_decrypted_buf = true;
                    self.read_encrypted(buf)
                } else {
                    self.already_read += r_len;
                    Err(ErrorKind::WouldBlock.into())
                }
            } else {
                Err(ErrorKind::WouldBlock.into())
            }
        }
    }

    fn poll_incoming(&mut self) -> Poll<(), io::Error> {
        let mut data = [0; 1536];
        loop {
            match self.read(&mut data) {
                Err(e) => match e.kind() {
                    ErrorKind::WouldBlock => {
                        return Ok(NotReady);
                    },
                    _ => {
                        return Err(e);
                    },
                },
                Ok(r_len) => {
                    if r_len == 0 {
                        return Ok(Ready(()));
                    }
                    self.incoming_sender.unbounded_send(data[..r_len].to_vec())
                        // .map_err(|_| Error::from_str("couldn't send incoming data").into())?;
                        .map_err(|_| io::Error::new(io::ErrorKind::Other, "couldn't send incoming data"))?;
                },
            }
        }
    }

    fn poll_outgoing(&mut self) -> Poll<(), ()> {
        loop {
            match try_ready!(self.outgoing_receiver.poll()) {
                None => {
                    return Ok(Ready(()));
                },
                Some(data) => {
                    self.write(&data).map_err(|_| ())?;
                },
            }
        }
    }
}

impl Future for EncryptedStream {
    type Error = io::Error;
    type Item = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.poll_outgoing()
            // .map_err(|_| Error::from_str("couldn't receive outgoing data").into())?;
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "couldn't receive incoming data"))?;

        self.poll_incoming()
    }
}

impl Read for EncryptedStream {
    fn read(&mut self, buf: &mut [u8]) -> std::result::Result<usize, io::Error> {
        if self.shared_secret.is_none() {
            match self.session_receiver.poll() {
                Ok(Async::Ready(session)) => {
                    *self.controller_id.lock().expect("couldn't access controller_id") = Some(session.controller_id);
                    self.shared_secret = Some(session.shared_secret);
                },
                _ => {
                    return self.stream.read(buf);
                },
            }
        }

        match self.read_decrypted(buf) {
            Ok(size) => Ok(size),
            Err(_) => match self.read_encrypted(buf) {
                Ok(size) => Ok(size),
                Err(_) => self.read_stream(buf),
            },
        }
    }
}

impl Write for EncryptedStream {
    fn write(&mut self, buf: &[u8]) -> std::result::Result<usize, io::Error> {
        if let Some(shared_secret) = self.shared_secret {
            let mut write_buf = BytesMut::from_buf(buf);

            while write_buf.len() > 1024 {
                let (aad, chunk, auth_tag) = encrypt_chunk(&shared_secret, &write_buf[..1024], &mut self.encrypt_count)
                    .map_err(|_| io::Error::new(io::ErrorKind::Other, "encryption failed"))?;
                self.stream.write_all(&aad)?;
                self.stream.write_all(&chunk)?;
                self.stream.write_all(&auth_tag)?;
                write_buf.advance(1024);
            }

            let (aad, chunk, auth_tag) = encrypt_chunk(&shared_secret, &write_buf, &mut self.encrypt_count)
                .map_err(|_| io::Error::new(io::ErrorKind::Other, "encryption failed"))?; // TODO - make this better
            self.stream.write_all(&aad)?;
            self.stream.write_all(&chunk)?;
            self.stream.write_all(&auth_tag)?;
            Ok(buf.len())
        } else {
            self.stream.write(buf)
        }
    }

    fn flush(&mut self) -> std::result::Result<(), io::Error> { self.stream.flush() }
}

impl AsyncRead for EncryptedStream {}

impl AsyncWrite for EncryptedStream {
    fn shutdown(&mut self) -> Poll<(), io::Error> { AsyncWrite::shutdown(&mut self.stream) }
}

fn decrypt_chunk(
    shared_secret: &[u8; 32],
    aad: &[u8],
    data: &[u8],
    auth_tag: &[u8],
    count: &mut u64,
) -> Result<Vec<u8>> {
    let mut decrypted_data = Vec::new();
    let read_key = compute_read_key(shared_secret);

    let mut nonce = vec![0; 4];
    let mut suffix = vec![0; 8];
    LittleEndian::write_u64(&mut suffix, *count);
    nonce.extend(suffix);
    *count += 1;

    chacha20_poly1305_aead::decrypt(&read_key, &nonce, aad, &data, auth_tag, &mut decrypted_data)?;

    Ok(decrypted_data)
}

fn encrypt_chunk(shared_secret: &[u8; 32], data: &[u8], count: &mut u64) -> Result<([u8; 2], Vec<u8>, [u8; 16])> {
    let mut encrypted_data = Vec::new();
    let write_key = compute_write_key(shared_secret);

    let mut nonce = vec![0; 4];
    let mut suffix = vec![0; 8];
    LittleEndian::write_u64(&mut suffix, *count);
    nonce.extend(suffix);
    *count += 1;

    let mut aad = [0; 2];
    LittleEndian::write_u16(&mut aad, data.len() as u16);

    let auth_tag = chacha20_poly1305_aead::encrypt(&write_key, &nonce, &aad, &data, &mut encrypted_data)?;

    Ok((aad, encrypted_data, auth_tag))
}

fn compute_read_key(shared_secret: &[u8; 32]) -> [u8; 32] {
    compute_key(shared_secret, b"Control-Write-Encryption-Key")
}

fn compute_write_key(shared_secret: &[u8; 32]) -> [u8; 32] {
    compute_key(shared_secret, b"Control-Read-Encryption-Key")
}

fn compute_key(shared_secret: &[u8; 32], info: &[u8]) -> [u8; 32] {
    let mut key = [0; 32];
    let salt = hmac::SigningKey::new(&digest::SHA512, b"Control-Salt");
    hkdf::extract_and_expand(&salt, shared_secret, &info, &mut key);
    key
}
