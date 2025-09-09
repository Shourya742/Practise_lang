use std::{fs::{File, OpenOptions}, io::{Read, Seek, Write}, path::PathBuf};

use crate::{error::Error, node_type::Offset, page_layout::PTR_SIZE};



pub struct Wal {
    file: File
}

impl Wal {
    pub fn new(parent_directory: PathBuf) -> Result<Self, Error> {
        let fd = OpenOptions::new().create(true).read(true).write(true).truncate(true).open(parent_directory.join("wal"))?;
        Ok(Self { file: fd })
    }

    pub fn get_root(&mut self) -> Result<Offset, Error> {
        let mut buf: [u8; PTR_SIZE] = [0x00; PTR_SIZE];
        let file_len = self.file.seek(std::io::SeekFrom::End(0))? as usize;
        let mut root_offset: usize = 0;
        if file_len > 0 {
            root_offset = (file_len / PTR_SIZE - 1) * PTR_SIZE;
        }
        self.file.seek(std::io::SeekFrom::Start(root_offset as u64))?;
        self.file.read_exact(&mut buf)?;
        Offset::try_from(buf)
    }

    pub fn set_root(&mut self, offset: Offset) -> Result<(), Error> {
        self.file.seek(std::io::SeekFrom::End(0))?;
        self.file.write_all(&offset.0.to_be_bytes())?;
        Ok(())
    }
}