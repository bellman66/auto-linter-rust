#![allow(non_snake_case)]

mod parse;

use std::path::Path;
use crate::parse::java_parser::context;
use crate::parse::java_parser::data::dto::Create;

fn main() {
    let path = Path::new("/Users/june/workspace/auto-linter-rust/target.java");

    let lines = context::read_file_by_path(path);

    let (header, body) = context::split_header_body(&lines);

    println!("header : {:?}", context::extract_header_line(header.get_content()));
}