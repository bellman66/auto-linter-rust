
pub trait Create {
    fn create(content: String) -> Self;
    fn get_content(&self) -> &str;
}

#[derive(Debug)]
pub struct Header {
    content: String
}

impl Create for Header {
    fn create(content: String) -> Self {
        Header {
            content
        }
    }

    fn get_content(&self) -> &str {
        &self.content
    }
}

#[derive(Debug)]
pub struct Body {
    content: String
}

impl Create for Body {
    fn create(content: String) -> Self {
        Body {
            content
        }
    }

    fn get_content(&self) -> &str {
        &self.content
    }
}