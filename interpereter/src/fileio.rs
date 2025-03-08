use std::fs::File;
use std::io;
use std::io::Read;

pub struct FilePos{
    row: usize,
    col: usize
}

pub struct InterpereterUnit{
    contents: [u8]
}

impl InterpereterUnit{
    fn open_file(){
        let mut tmp = File::open(&path)?;

        let tmpstr = String::new();
        tmp.read(&mut self.contents)?;
        return Ok(self);
    }

    fn get_contents() -> [u8]{
        return self.contents;
    }
}