use std::{fs::{File, OpenOptions}, io::{Read, Seek, Write}, path::Path};

use crate::{error::Error, node_type::Offset, page::Page, page_layout::PAGE_SIZE};



pub struct Pager {
    file: File,
    curser: usize
}

impl Pager {
    pub fn new(path: &Path) -> Result<Pager, Error> {
        let fd = OpenOptions::new().create(true).read(true).write(true).truncate(true).open(path)?;
        Ok(Pager { file: fd, curser: 0 })
    }

    pub fn get_page(&mut self, offset: &Offset) -> Result<Page, Error> {
        let mut page: [u8; PAGE_SIZE] = [0x00; PAGE_SIZE];
        self.file.seek(std::io::SeekFrom::Start(offset.0 as u64))?;
        self.file.read_exact(&mut page)?;
        Ok(Page::new(page))
    }

    pub fn write_page(&mut self, page: Page) -> Result<Offset, Error> {
        self.file.seek(std::io::SeekFrom::Start(self.curser as u64))?;
        self.file.write_all(&page.get_data())?;
        let res = Offset(self.curser);
        self.curser += PAGE_SIZE;
        Ok(res)
    }

    pub fn write_page_at_offset(&mut self, page: Page, offset: &Offset) -> Result<(), Error> {
        self.file.seek(std::io::SeekFrom::Start(offset.0 as u64))?;
        self.file.write_all(&page.get_data())?;
        Ok(())
    }
}