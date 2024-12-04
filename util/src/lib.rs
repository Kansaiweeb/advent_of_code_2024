use std::fs::read_to_string;

pub fn read_input(path: &str) -> Result<String, std::io::Error> {
    read_to_string(path)
}
