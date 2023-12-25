
use std::fs::{File};
use std::string::String;
use std::io::{BufReader, Read};

pub fn read() -> String {
    let path = "C:\\workspace\\auto-linter-rust\\target.java";

    let file:File = File::open(path).expect("Not Found File");
    let mut reader:BufReader<File> = BufReader::new(file);

    let mut result = String::new();
    reader.read_to_string(&mut result).expect("Failed to Parse");

    result
}
