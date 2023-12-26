#![allow(non_snake_case)]

mod parse;

use std::path::Path;
use crate::parse::java_parser::context;

fn main() {
    // launch the dioxus app in a webview
    // dioxus_desktop::launch(App);

    let path = Path::new("/Users/june/workspace/auto-linter-rust/target.java");

    let string = context::read_file_by_path(path);

    println!("{:?}", string)
}
