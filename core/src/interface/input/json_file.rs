use std::fs;
use std::io;
/// This function reads a JSON file from the specified path and returns its contents as a String.
/// ```
///json_file("sample.json");
/// ```
pub fn json_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}
