use std::{io::Write, marker::PhantomData, net::TcpStream, sync::Arc};

use serde::Serialize;

pub struct MessageWriter<T> {
    sink: Arc<TcpStream>,
    _phantom: PhantomData<T>,
}

impl<T: Serialize> MessageWriter<T> {
    pub fn new(sink: Arc<TcpStream>) -> Self {
        Self {
            sink,
            _phantom: Default::default(),
        }
    }

    // pub fn inner(&self) -> &W {
    //     &self.sink
    // }

    pub fn send(&mut self, message: T) -> anyhow::Result<()> {
        let mut serialized = serde_json::to_vec(&message)?;
        serialized.extend_from_slice(&b"\n"[..]);
        self.sink.as_ref().write(&serialized)?;
        self.sink.as_ref().flush()?;
        Ok(())
    }
}
