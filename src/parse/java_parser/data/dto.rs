enum HeaderType {
    Package,
    Import,
    StaticImport
}

pub struct Header {
    package: String,
    import_group: Vec<String>,
    static_import_group: Vec<String>
}

impl Header {

    const NEXT_LINE: &'static str = "\n";

    pub fn create(content: String) -> Self {
        let mut package = String::new();
        let mut import_group: Vec<String> = vec![];
        let mut static_import_group: Vec<String> = vec![];

        content.split(';')
            .map(|value| value.trim())
            .filter(|value| value.starts_with("package ") || value.starts_with("import ") || value.starts_with("import static "))
            .for_each(|value| {
                if value.starts_with("package ") {
                    package = value.to_string();
                } else if value.starts_with("import ") {
                    import_group.push(value.to_string())
                } else if value.starts_with("import static ") {
                    import_group.push(value.to_string())
                }
            });

        Header {
            package,
            import_group,
            static_import_group
        }
    }

    pub fn get_content(&self) -> String {
        let mut result = String::new();

        self.add_line(&mut result, &self.package);
        self.add_line_group(&mut result, &self.import_group);
        self.add_line_group(&mut result, &self.static_import_group);

        result
    }

    fn add_line(&self, builder: &mut String, line: &str) {
        if line.is_empty() {
            return;
        }

        builder.push_str(line);
        builder.push_str(Self::NEXT_LINE);
    }

    fn add_line_group(&self, builder: &mut String, line_group: &Vec<String>) {
        if line_group.is_empty() {
            return;
        }

        builder.push_str(Self::NEXT_LINE);

        line_group.iter()
            .for_each(|value| {
                self.add_line(builder, value);
            })
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
