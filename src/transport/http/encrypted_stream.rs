use std::io::{self, Read, Write, ErrorKind};
use std::cmp::min;
use futures::{Async, Future, Poll};
use futures::sync::oneshot;
use tokio_core::net::TcpStream;
use tokio_io::{AsyncRead, AsyncWrite};
use ring::{hkdf, hmac, digest};
use chacha20_poly1305_aead;
use bytes::BytesMut;
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};

pub struct EncryptedStream {
    stream: TcpStream,
    secret_receiver: oneshot::Receiver<[u8; 32]>,
    shared_secret: Option<[u8; 32]>,
    decrypt_count: u64,
    encrypt_count: u64,
    encrypted_buf: BytesMut,
    decrypted_buf: BytesMut,
}

impl EncryptedStream {
    pub fn new(stream: TcpStream) -> (EncryptedStream, oneshot::Sender<[u8; 32]>) {
        let (sender, receiver) = oneshot::channel();
        (EncryptedStream {
            stream,
            secret_receiver: receiver,
            shared_secret: None,
            decrypt_count: 0,
            encrypt_count: 0,
            encrypted_buf: BytesMut::with_capacity(1042),
            decrypted_buf: BytesMut::with_capacity(1024),
        }, sender)
    }

    fn read_decrypted(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        let d_len = self.decrypted_buf.len();
        if d_len > 0 {
            let len = min(buf.len(), d_len);
            &buf[..len].copy_from_slice(&self.decrypted_buf[..len]);
            self.decrypted_buf.advance(len);
            return Ok(len);
        }
        Err(ErrorKind::WouldBlock.into())
    }

    fn read_encrypted(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        let e_len = self.encrypted_buf.len();
        if e_len > 18 {
            let aad = LittleEndian::read_u16(&self.encrypted_buf[..2]) as usize;
            if aad >= e_len - 16 {
                let decrypted = decrypt_chunk(
                    &self.shared_secret.unwrap(),
                    self.encrypted_buf[2..aad].to_vec(),
                    &self.encrypted_buf[aad..aad + 16],
                    &mut self.decrypt_count,
                );
                self.decrypted_buf.extend(decrypted);
                self.encrypted_buf.advance(aad + 16);

                return self.read_decrypted(buf);
            }
        }
        Err(ErrorKind::WouldBlock.into())
    }

    fn read_stream(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        // TODO - extend buffer instead of overwriting it
        let r_len = self.stream.read(&mut self.encrypted_buf).unwrap();
        if r_len > 18 {
            return self.read_encrypted(buf);
        }
        Err(ErrorKind::WouldBlock.into())
    }
}

impl Read for EncryptedStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        if self.shared_secret.is_none() {
            match self.secret_receiver.poll() {
                Ok(Async::Ready(shared_secret)) => {
                    self.shared_secret = Some(shared_secret);
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

fn decrypt_chunk(shared_secret: &[u8; 32], data: Vec<u8>, auth_tag: &[u8], count: &mut u64) -> Vec<u8> {
    let mut decrypted_data = Vec::new();
    let read_key = compute_read_key(shared_secret);
    let mut nonce = [0; 12];
    LittleEndian::write_u64(&mut nonce, count.clone());
    *count += 1;
    chacha20_poly1305_aead::decrypt(&read_key, &nonce, &[], &data, auth_tag, &mut decrypted_data).unwrap();
    decrypted_data
}

fn compute_read_key(shared_secret: &[u8; 32]) -> [u8; 32] {
    compute_key(shared_secret, b"Control-Write-Encryption-Key".to_vec())
}

fn compute_write_key(shared_secret: &[u8; 32]) -> [u8; 32] {
    compute_key(shared_secret, b"Control-Read-Encryption-Key".to_vec())
}

fn compute_key(shared_secret: &[u8; 32], info: Vec<u8>) -> [u8; 32] {
    let mut key = [0; 32];
    let salt = hmac::SigningKey::new(&digest::SHA512, b"Control-Salt");
    hkdf::extract_and_expand(&salt, shared_secret, &info, &mut key);
    key
}
