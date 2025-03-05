#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_file() {
        assert_eq!("Hello world!", read("hwtest.txt"))
    }
}