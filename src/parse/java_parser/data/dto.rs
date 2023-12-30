use std::collections::hash_map::Values;
use crate::parse::java_parser::context::extract_header_line;


#[derive(Debug)]
pub struct Header {
    value_array: Vec<String>
}

impl Header {

    pub fn create(content: String) -> Self {
        Header {
            value_array: extract_header_line(content)
        }
    }

    fn extract_header_line(content: String) -> Vec<String> {
        content.split(';')
            .filter(|value| value.contains("package") || value.contains("import"))
            .map(|value| String::from(value.trim()))
            .collect::<Vec<String>>()
    }

    pub fn get_content(&self) -> &Vec<String> {
        &self.value_array
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
