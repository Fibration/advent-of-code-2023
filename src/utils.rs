use std::fs;

pub fn read(filename: String) -> String {
    let contents = fs::read_to_string(filename).expect("Should have been able to read the file");
    contents
}
