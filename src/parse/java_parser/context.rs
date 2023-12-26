
use std::fs::{File};
use std::string::String;
use std::io::{BufReader, Read};
use std::path::Path;

pub fn read_file_by_path(path: &Path) -> String {
    let file:File = File::open(path).expect("Not Found File");
    let mut reader:BufReader<File> = BufReader::new(file);

    let mut result = String::new();
    reader.read_to_string(&mut result).expect("Failed to Parse");

    result
}
