
trait Create {
    fn create(content: String) -> Self;
}

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