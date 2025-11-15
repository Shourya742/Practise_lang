use std::{fs::File, marker::PhantomData};

struct Reading;


struct Eof;

struct File2<State> {
    inner: File,
    _state: PhantomData<State>
}


enum ReadResult {
    Read(File2<Reading>, Vec<u8>),
    Eof(File2<Eof>)
}


impl File2<Reading> {
    pub fn open(path: String) -> Option<File2<Reading>> {todo!()}

    pub fn read(self) -> ReadResult {
        match self.inner.read() {
        // 3. Access to `ReadingFile` is only given back if not at EOF
        Some(bytes) => ReadResult::Read(self, bytes)
        None => ReadResult::Eof(File2 { inner: self.inner, _state: PhantomData })
    }
    }

    pub fn close(self) {
        self.inner.close();
      }
}

impl File2<Eof> {
    pub fn close(self) {
      self.inner.close();
    }
  }
  