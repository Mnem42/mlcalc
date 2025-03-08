use std::fs;

pub fn read(path: &str) -> String {
    fs::read_to_string(path)
        .expect("Should have been able to read the file")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_file() {
        assert_eq!("Hello world!", read("src\\test-files\\hwtest.txt"))
    }

}
