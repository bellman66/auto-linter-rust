
pub trait Create {
    fn create(content: String) -> Self;
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
}