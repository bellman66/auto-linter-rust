
#[derive(Debug)]
pub struct Header {
    is_package_contains: bool,
    value_array: Vec<String>
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

    fn extract_header_line(content: String) -> Vec<String> {
        content.split(';')
            .filter(|value| value.contains("package") || value.contains("import"))
            .map(|value| String::from(value.trim()))
            .collect::<Vec<String>>()
    }

    pub fn get_content(&self) -> String {
        let mut result = String::new();
        let mut start = 0;
        let end = self.value_array.len();

        if self.is_package_contains {
            start = 1;
            result.push_str(&self.value_array[0]);
            result.push_str(Self::NEXT_LINE)
        }

        for idx in start..end {
            result.push_str(&self.value_array[idx]);
            result.push_str(Self::NEXT_LINE)
        }
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
