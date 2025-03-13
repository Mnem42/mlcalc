pub enum Item {
    Line(String),
    Newline,
    EOF,
}

struct StringSlice {}

impl Iterator for StringSlice {
    type Item = Item;

    fn next(&mut self) -> Option<Item> {
        todo!("This isn't actually implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
