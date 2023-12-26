
use std::fs::{File};
use std::string::String;
use std::io::{BufReader, Read};
use std::path::Path;
use crate::parse::java_parser::data::dto::{Header, Body};

const INTERFACE_TARGET: &'static str = "interface";

pub fn read_file_by_path(path: &Path) -> String {
    let file:File = File::open(path).expect("Not Found File");
    let mut reader:BufReader<File> = BufReader::new(file);

    let mut result = String::new();
    reader.read_to_string(&mut result).expect("Failed to Parse");

    result
}

pub fn split_header_body(lines: &str) -> (Header, Body) {
    let vec = lines.split(INTERFACE_TARGET).collect::<Vec<&str>>();

}