use std::fs::File;
use std::path::Path;
use std::io::{Read, Split};
use std::str::SplitWhitespace;

pub struct FilePos{
    row: usize,
    col: usize
}

pub struct InterpereterUnit{
    contents: String
}

impl InterpereterUnit{
    pub fn new()->InterpereterUnit{
        InterpereterUnit{contents:String::new()}
    }

    pub fn open_file(&mut self,pathname: &Path) -> Result<&InterpereterUnit,std::io::Error>{
        let mut tmp = File::open(&pathname)?;

        tmp.read_to_string(&mut self.contents)?;
        return Ok(self);
    }

    pub fn str_tokenise(&self) -> SplitWhitespace{
        return self.contents.split_whitespace();
    }

    pub fn get_contents_mut(&self) -> String{
        self.contents.clone()
    }

    pub fn get_contents(&self) -> &String{
        &self.contents
    }
}