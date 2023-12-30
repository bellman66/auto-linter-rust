use std::collections::HashMap;
use crate::parse::java_parser::data::dto::HeaderPrevious::{Import, Package};

#[derive(Eq, Hash, PartialEq)]
enum HeaderPrevious {
    Package,
    Import,
    StaticImport
}

pub struct Header {
    is_package_contains: bool,
    value_array: HashMap<HeaderPrevious, String>
}

impl Header {

    const NEXT_LINE: &'static str = "\n";

    pub fn create(content: String) -> Self {
        Header {
            is_package_contains: Self::check_package_previous(&content),
            value_array: Self::extract_header_line(content)
        }
    }

    fn check_package_previous(content: &String) -> bool {
        content.contains("package ")
    }

    fn extract_header_line(content: String) -> HashMap<HeaderPrevious, String> {
        let mut result: HashMap<HeaderPrevious, String> = HashMap::<HeaderPrevious, String>::new();

        content.split(';')
            .map(|value| value.trim())
            .for_each(|line| {
                if line.contains("package ") {
                    result.insert(Package, line.to_string());
                }
                else if line.contains("import ") {
                    result.insert(Import, line.to_string());
                }
            });
        result
    }

    pub fn get_content(&self) -> String {
        let mut result = String::new();



        result
    }
}

#[derive(Debug)]
pub struct Body {
    content: String
}

impl Body {
    pub fn create(content: String) -> Self {
        Body {
            content
        }
    }
}
