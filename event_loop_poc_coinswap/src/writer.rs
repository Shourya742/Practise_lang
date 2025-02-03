use std::{io::Write, marker::PhantomData};

use serde::Serialize;



pub struct MessageWriter<T, W> {
    sink: W,
    _phantom: PhantomData<T>
}

impl<T:Serialize, W: Write> MessageWriter<T, W> {
    pub fn new(sink: W) -> Self {
        Self {
            sink,
            _phantom: Default::default()
        }
    }

    pub fn inner(&self) -> &W {
        &self.sink
    }

    pub fn send(&mut self, message: T) -> anyhow::Result<()> {
        let serialized = serde_json::to_vec(&message)?;
        self.sink.write_all(&serialized)?;
        self.sink.write_all(b"\n")?;
        self.sink.flush()?;
        Ok(())
    }
 }