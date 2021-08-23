
pub mod markdown_handler {
    use std::path::{Path, PathBuf};

    use pulldown_cmark::*;
    use regex::Regex;
    use crate::filesystem_handler::filesystem_handler::FilesystemHandler;
    use std::error::Error;
    use std::fs;

    pub const MARKER_CHARACTER: char = '⁂';
    lazy_static! {
        static ref INLINE_CODE_REGEX: Regex = Regex::new(r"\$\$([^\$]*)\$\$").unwrap();
    }
    pub struct MarkdownHandler {
        markdown_string: String,
        pub do_refresh: bool,
    }
    impl MarkdownHandler {
        pub fn new(input: &str) -> MarkdownHandler {
            MarkdownHandler {
                markdown_string: MarkdownHandler::transform_input(input),
                do_refresh: true,
            }
        }
        pub fn load_markdown_from_file(&mut self, input: &Path) -> Result<(),std::io::Error> {
            if !FilesystemHandler::verify_file_argument(Some(input.to_path_buf())) {
                eprintln!("Please pass a valid .mdl file path as an argument");
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput,"File not valid or not .mdl"));
            }
            let file = fs::read_to_string(&input)?;
            self.load_markdown(&file);
            return Ok(());
        }
        pub fn load_markdown(&mut self, input: &str) {
            self.markdown_string = MarkdownHandler::transform_input(input);
            self.do_refresh = true
        }
        fn transform_input(input: &str) -> String {
            INLINE_CODE_REGEX
                .replace_all(&input, format!("`{}$1`", MARKER_CHARACTER).as_str())
                .to_string()
        }
        pub fn gen_parser<'a>(&'a self) -> Box<dyn Iterator<Item = Event<'a>> + 'a> {
            let parser = Parser::new_ext(&self.markdown_string, Options::ENABLE_STRIKETHROUGH);
            let parser = parser.map(|event| match event {
                Event::Start(Tag::Image(linktype, dest, text)) => {
                    if linktype == LinkType::Inline {
                        let dest = Path::new(dest.as_ref()).file_name();
                        let mut new_path = "http://127.0.0.1:8080/".to_string();
                        if let Some(file_name) = dest {
                            new_path.push_str(file_name.to_str().unwrap());
                        }
                        Event::Start(Tag::Image(linktype, CowStr::from(new_path), text))
                    } else {
                        Event::Start(Tag::Image(linktype, dest, text))
                    }
                }
                _ => event,
            });
            Box::new(parser)
        }
    }
}
