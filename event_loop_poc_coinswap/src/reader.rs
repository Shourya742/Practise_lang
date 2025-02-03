use std::{io::Read, marker::PhantomData, net::TcpStream, sync::Arc};

use serde::de::DeserializeOwned;

const MAX_MESSAGE_SIZE: usize = 256;

pub struct MessageReader<T> {
    pub stream: Arc<TcpStream>,
    buffer: Vec<u8>,
    loaded: usize,
    _phantom: PhantomData<T>,
}

impl<T: DeserializeOwned> MessageReader<T> {
    pub fn new(stream: Arc<TcpStream>) -> Self {
        Self {
            buffer: vec![0; MAX_MESSAGE_SIZE * 4],
            loaded: 0,
            stream,
            _phantom: Default::default(),
        }
    }

    pub fn recv(&mut self) -> Option<std::io::Result<T>> {
        loop {
            if let Some(position) = self.buffer[..self.loaded].iter().position(|c| *c == b'\n') {
                let msg = &self.buffer[..position];
                let msg: T = match serde_json::from_slice(msg) {
                    Ok(msg) => msg,
                    Err(error) => return Some(Err(error.into())),
                };
                self.buffer.copy_within(position + 1.., 0);
                self.loaded -= position + 1;
                return Some(Ok(msg));
            }

            if self.loaded >= MAX_MESSAGE_SIZE {
                return Some(Err(std::io::Error::new(
                    std::io::ErrorKind::OutOfMemory,
                    "Too large message",
                )));
            }

            let read_bytes = match self.stream.as_ref().read(&mut self.buffer[self.loaded..]) {
                Ok(b) => b,
                Err(error) => return Some(Err(error)),
            };

            if read_bytes == 0 {
                break;
            }
            self.loaded += read_bytes;
        }

        None
    }
}
