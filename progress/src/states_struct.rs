use std::{fs::File, io::Read};




struct ReadingFile {inner: File}

struct EofFile {inner: File}


enum ReadResult {
    Read(ReadingFile, Vec<u8>),
    Eof(EofFile)
}

impl ReadingFile {
    pub fn open(path: String) -> Option<ReadingFile> {
        todo!()
    }

    pub fn read(mut self) -> ReadResult {
        let mut buf = [0; 1024];
        match self.inner.read(&mut buf) {
            Ok(bytes) => ReadResult::Read(self, buf.to_vec()),
            Err(_) => ReadResult::Eof(EofFile { inner: self.inner })
        }
    }

    pub fn close(self) {
        self.inner.close()
    }
}


impl EofFile {
    pub fn close(self) {
        self.inner.close();
    }
}

