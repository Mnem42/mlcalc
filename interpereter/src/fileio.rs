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
    //Helper function for initialising
    pub fn new()->InterpereterUnit{
        InterpereterUnit{contents:String::new()}
    }

    //Opens a file and sets the contents to the file contents
    pub fn open_file(&mut self,pathname: &Path) -> Result<&InterpereterUnit,std::io::Error>{
        let mut tmp = File::open(&pathname)?;

        tmp.read_to_string(&mut self.contents)?;
        return Ok(self);
    }

    //Tokenises and returns an iterator
    pub fn str_tokenise(&self) -> SplitWhitespace{
        return self.contents.split_whitespace();
    }

    //returns an copy of the contents (note:uses clone)
    pub fn get_contents_copy(&self) -> String{
        self.contents.clone()
    }
    //returns an mutable reference to the contents (note:risky)
    pub fn get_contents_mut(&mut self) -> &mut String{
        &mut self.contents
    }
    //returns an immutable reference to the contents
    pub fn get_contents(&self) -> &String{
        &self.contents
    }
}