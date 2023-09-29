
use std::fs::{File};
use std::string::String;
use std::io::{BufReader, Read};

use xml::reader::{EventReader, XmlEvent};

struct LineLength {
    fileExtensions: String,
    max: i8,
    ignorePattern: String
}

impl LineLength {
    pub fn new() -> Self {
        Self {
            fileExtensions: String::new(),
            max: 0,
            ignorePattern: String::new()
        }
    }

    pub fn set_fileExtensions(&mut self, fileExtensions: String) {
        self.fileExtensions = fileExtensions;
    }
    pub fn set_max(&mut self, max: i8) {
        self.max = max;
    }
    pub fn set_ignorePattern(&mut self, ignorePattern: String) {
        self.ignorePattern = ignorePattern;
    }
}

enum XmlModuleName {
    LineLength(String),
}

struct LinterContext {
    lineLength: LineLength
}

impl LinterContext {
    pub fn new() -> Self {
        Self {
            lineLength: LineLength::new()
        }
    }
}

pub fn open() {
    let path = "/Users/youn/develop/workspace/java-optimize/src/main/java/checker/CheckUtil.java";

    let mut file:File = File::open(path).expect("Not Found File");
    let mut file:BufReader<File> = BufReader::new(file);

    let mut content = String::new();
    file.read_to_string(&mut content).expect("Failed to Parse");

    // 1. Set Xml Parser
    let xml_path = "/Users/youn/develop/google_checks.xml";

    let mut file:File = File::open(xml_path).expect("Not Found File");
    let mut file:BufReader<File> = BufReader::new(file);

    let mut parser = EventReader::new(file);
    let mut depth = 0;

    let mut context = LinterContext::new();

    let mut module_name: String = String::new();
    loop {
        match parser.next() {
            Ok(element) => {
                match element {
                    XmlEvent::StartDocument { .. } => {
                        println!("start xml --- ")
                    }
                    XmlEvent::EndDocument => {
                        print!("end xml --- ");
                        break;
                    }
                    XmlEvent::ProcessingInstruction { .. } => {}
                    XmlEvent::StartElement { name, attributes, .. } => {
                        if attributes.is_empty() {
                            continue;
                        }

                        if name.local_name.eq("module") {
                            module_name = String::from(&attributes[0].value);
                        }
                        else {
                            let target_name = &attributes[0].value;

                            match target_name.as_str() {
                                "fileExtensions" => {
                                    if module_name.eq("LineLength") {
                                        context.lineLength.set_fileExtensions(String::from(&attributes[1].value));
                                    }
                                },
                                &_ => {}
                            }
                        }
                    }
                    XmlEvent::EndElement { name } => {
                        if name.local_name.eq("module") {
                            // End Module
                        }
                    }
                    XmlEvent::CData(_) => {}
                    XmlEvent::Comment(_) => {}
                    XmlEvent::Characters(_) => {}
                    XmlEvent::Whitespace(_) => {}
                }
            }
            Err(_) => {
                println!("File Xml Error");
                break;
            }
        }
    }

    // 2. Apply String Content

    // print!("{}", xml_content);
}