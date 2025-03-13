use std::path::Path;
use std::str::SplitWhitespace;

pub struct FilePos {
    row: usize,
    col: usize,
}

pub struct InterpereterUnit {
    pub(crate) contents: String,
}

impl InterpereterUnit {
    pub fn new() -> InterpereterUnit {
        InterpereterUnit {
            contents: String::new(),
        }
    }

    /// Opens a file and sets the contents to the file contents
    pub fn open_file(&mut self, path: &Path) -> Result<&mut InterpereterUnit, std::io::Error> {
        self.contents = std::fs::read_to_string(path)?;

        Ok(self)
    }

    pub fn str_tokenise(&self) -> SplitWhitespace {
        self.contents.split_whitespace()
    }
}
