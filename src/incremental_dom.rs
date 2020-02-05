pub mod incremental_dom {
    use pulldown_cmark::{Event, Tag, Alignment, CowStr};
    use std::collections::HashMap;
    use std::borrow::Cow;

    enum TableState {
        Head,
        Body,
    }
    pub fn push_incremental_dom<'a,I>(s: &'a mut String, iter: I)
    where I: Iterator<Item =Event<'a>>
    {
       IncrementalDomWriter::new(s,iter).run();
    }
    struct IncrementalDomWriter<'a,I> {
        iter: I,
        incremental_dom: &'a mut String,
        table_state: TableState,
        table_alignments: Vec<Alignment>,
        table_cell_index: usize,
        numbers: HashMap<CowStr<'a>, usize>,
        do_latex_render: bool
    }
    impl<'a,I> IncrementalDomWriter<'a,I> where I: Iterator<Item = Event<'a>> {
        pub fn new(s: &'a mut String, iter: I) -> IncrementalDomWriter<I>
        {
           Self {
               iter,
               incremental_dom: s,
               table_state: TableState::Head,
               table_alignments: vec![],
               table_cell_index: 0,
               numbers: HashMap::new(),
               do_latex_render: false
           }
        }
        fn escape_without_quotes(s: &str) -> String {
            s.replace("\n","").replace("\\","\\\\").replace("\"","\\\"").replace("\'","\\'")
        }
        fn run(&mut self)
        {
            self.incremental_dom.clear();
            while let Some(event) = self.iter.next() {
                match event {
                    Event::Start(tag) => {
                        let start_string = self.start_tag(tag);
                        self.incremental_dom.push_str(&start_string);
                    },
                    Event::End(tag) => {
                        let end_string = self.end_tag(tag);
                        self.incremental_dom.push_str(&end_string);
                    },
                    Event::Text(text) => {
                        let text = Self::escape_without_quotes(&text);
                        if self.do_latex_render {
                            self.incremental_dom.push_str(&format!(r#"html("${{katex.renderToString('{}', {{displayMode:true}}).replace(/"/g, '\\\"')}}");"#,text))
                        }
                        else {
                            self.incremental_dom.push_str(&format!("window.IncrementalDOM.text('{}');",text));
                        }
                    },
                    Event::Code(text) => {
                        let text = Self::escape_without_quotes(&text);
                        if let Some(text) = text.strip_prefix(crate::markdown_handler::markdown_handler::MARKER_CHARACTER) {
                            self.incremental_dom.push_str(&format!(r#"html("<span class='inline-math'>${{katex.renderToString('{}', {{displayMode:false}}).replace(/"/g, '\\\"')}}</span>");"#,text))
                        }
                        else {
                            self.incremental_dom.push_str("window.IncrementalDOM.elementOpen('code',null,null);");
                            self.incremental_dom.push_str(&format!("window.IncrementalDOM.text('{}');",text));
                            self.incremental_dom.push_str("window.IncrementalDOM.elementClose('code');");
                        }
                    },
                    Event::Html(text) => {
                        let text = Self::escape_without_quotes(&text);
                        self.incremental_dom.push_str(&format!("html('{}');",text));
                    },
                    Event::SoftBreak => {
                        self.incremental_dom.push_str("window.IncrementalDOM.elementVoid('br',null,null);");
                    },
                    Event::HardBreak => {
                        self.incremental_dom.push_str("window.IncrementalDOM.elementVoid('br',null,null);");
                    },
                    Event::Rule => {
                        self.incremental_dom.push_str("window.IncrementalDOM.elementVoid('hr',null,null);");
                    },
                    Event::FootnoteReference(name) => {
                        let encoded_name = htmlescape::encode_attribute(&name);
                        let len = self.numbers.len() + 1;
                        self.incremental_dom.push_str("window.IncrementalDOM.elementOpen('sup',null,null,'class','footnote-reference');");
                        self.incremental_dom.push_str(&format!("window.IncrementalDOM.elementOpen('a', null,null,'href','#{}",encoded_name));
                        let number = *self.numbers.entry(name).or_insert(len);
                        self.incremental_dom.push_str(&format!("window.IncrementalDOM.text('{}');",number));
                        self.incremental_dom.push_str("window.IncrementalDOM.elementClose('a');window.IncrementalDOM.elementClose('sup');");
                    },
                    Event::TaskListMarker(true) => {
                        self.incremental_dom.push_str("window.IncrementalDOM.elementVoid('input',null,null,'disabled','','type','checkbox','checked','');")
                    },
                    Event::TaskListMarker(false) => {
                        self.incremental_dom.push_str("window.IncrementalDOM.elementVoid('input',null,null,'disabled','','type','checkbox');")
                    }
                }
            }
        }
        fn start_tag(&mut self, tag: Tag<'a>) -> String {
            match tag {
                Tag::Paragraph => {
                    "window.IncrementalDOM.elementOpen('p',null,null);".to_string()
                },
                Tag::CodeBlock(info) => {
                    let lang = info.split(' ').next().unwrap();
                    if lang == "latex" {
                        self.do_latex_render = true;
                    }
                    format!("window.IncrementalDOM.elementOpen('pre',null,null);window.IncrementalDOM.elementOpen('code',null,null,'class','language-{}');",lang)
                },
                Tag::Image(linktype,url,title) => {
                    format!("window.IncrementalDOM.elementOpen('img',null,null,'src','{}','alt','{}','title','{}');",url,self.raw_text(),title) +
                        "window.IncrementalDOM.elementClose('img');"
                },
                Tag::BlockQuote => {
                    "window.IncrementalDOM.elementOpen('blockquote',null,null);".to_string()
                },
                Tag::Emphasis => {
                    "window.IncrementalDOM.elementOpen('em',null,null);".to_string()
                },
                Tag::FootnoteDefinition(name) => {
                    let encoded_name = htmlescape::encode_attribute(&name);
                    let len = self.numbers.len() + 1;
                    let number = *self.numbers.entry(name).or_insert(len);
                    format!("window.IncrementalDOM.elementOpen('div',null,null,'class','footnote-definition','id','{};",encoded_name)
                        + "window.IncrementalDOM.elementOpen('sup',null,null,'class','footnote-definition-label');"
                        + &format!("window.IncrementalDOM.text('{}');",number)
                        + "window.IncrementalDOM.elementClose('sup');"
                },
                Tag::Heading(size) => {
                    format!("window.IncrementalDOM.elementOpen('h{}',null,null);",size)
                },
                Tag::Item => {
                   "window.IncrementalDOM.elementOpen('li',null,null);".to_string()
                },
                Tag::Link(linktype,dest,title) => {
                    "".to_string() // No links
                },
                Tag::List(Some(1)) => {
                    "window.IncrementalDOM.elementOpen('ol',null,null);".to_string()
                },
                Tag::List(Some(start)) => {
                    format!("window.IncrementalDOM.elementOpen('ol',null,null,'start','{}');",start)
                },
                Tag::List(None) => {
                    "window.IncrementalDOM.elementOpen('ul',null,null);".to_string()
                },
                Tag::Strikethrough => {
                    "window.IncrementalDOM.elementOpen('del',null,null);".to_string()
                },
                Tag::Strong => {
                    "window.IncrementalDOM.elementOpen('strong',null,null);".to_string()
                },
                Tag::Table(alignment) => {
                    self.table_alignments = alignment;
                    "window.IncrementalDOM.elementOpen('table',null,null);".to_string()
                },
                Tag::TableHead => {
                    self.table_state = TableState::Head;
                    self.table_cell_index = 0;
                    concat!("window.IncrementalDOM.elementOpen('thead',null,null);","window.IncrementalDOM.elementOpen('tr',null,null);").to_string()
                },
                Tag::TableRow => {
                    self.table_cell_index = 0;
                    "window.IncrementalDOM.elementOpen('tr',null,null);".to_string()
                },
                Tag::TableCell => {
                    let tag = match self.table_state {
                        TableState::Head => "th",
                        TableState::Body => "td"
                    };
                    let align = match self.table_alignments.get(self.table_cell_index) {
                        Some(&Alignment::Left) => "left",
                        Some(&Alignment::Center) => "center",
                        Some(&Alignment::Right) => "right",
                        _ => ""
                    };
                    format!("window.IncrementalDOM.elementOpen('{}',null,null,'align','{}');",tag,align)
                }
            }
        }
        fn end_tag(&mut self, tag: Tag<'a>) -> String {
            match tag {
                Tag::Paragraph => {
                    "window.IncrementalDOM.elementClose('p');".to_string()
                },
                Tag::Heading(size) => {
                    format!("window.IncrementalDOM.elementClose('h{}');",size)
                },
                Tag::Table(_) => {
                    "window.IncrementalDOM.elementClose('tbody');window.IncrementalDOM.elementClose('table');".to_string()
                },
                Tag::TableHead => {
                    self.table_state = TableState::Body;
                    "window.IncrementalDOM.elementClose('tr');window.IncrementalDOM.elementClose('thead');window.IncrementalDOM.elementOpen('tbody',null,null);".to_string()

                },
                Tag::TableRow => {
                    "window.IncrementalDOM.elementClose('tr');".to_string()
                },
                Tag::TableCell => {
                    self.table_cell_index += 1;
                    match self.table_state {
                        TableState::Head => {
                            "window.IncrementalDOM.elementClose('th');"
                        }
                        TableState::Body => {
                            "window.IncrementalDOM.elementClose('td');"
                        }
                    }.to_string()
                },
                Tag::BlockQuote => {
                    "window.IncrementalDOM.elementClose('blockquote');".to_string()
                },
                Tag::CodeBlock(_) => {
                    self.do_latex_render = false;
                    "window.IncrementalDOM.elementClose('code');window.IncrementalDOM.elementClose('pre');".to_string()
                },
                Tag::List(Some(_)) => {
                    "window.IncrementalDOM.elementClose('ol');".to_string()
                },
                Tag::List(None) => {
                    "window.IncrementalDOM.elementClose('ul');".to_string()
                },
                Tag::Item => {
                    "window.IncrementalDOM.elementClose('li');".to_string()
                },
                Tag::Emphasis => {
                    "window.IncrementalDOM.elementClose('em');".to_string()
                },
                Tag::Strong => {
                    "window.IncrementalDOM.elementClose('strong');".to_string()
                },
                Tag::Strikethrough => {
                    "window.IncrementalDOM.elementClose('del');".to_string()
                },
                Tag::Link(_,_,_) => {
                    "".to_string() // No links
                },
                Tag::Image(_,_,_) => {
                    "".to_string() // handled in start
                },
                Tag::FootnoteDefinition(_) => {
                    "window.IncrementalDOM.elementClose('div');".to_string()
                }
            }
        }
        // run raw text, consuming end tag
        fn raw_text(&mut self) -> String {
            let mut ret_text:String = "".to_string();
            let mut nest = 0;
            while let Some(event) = self.iter.next() {
                match event {
                    Event::Start(_) => nest += 1,
                    Event::End(_) => {
                        if nest == 0 {
                            break;
                        }
                        nest -= 1;
                    }
                    Event::Html(text) | Event::Code(text) | Event::Text(text) => {
                        let escaped = htmlescape::encode_attribute(&text);
                        ret_text.push_str(&escaped);
                    }
                    Event::SoftBreak | Event::HardBreak | Event::Rule => {
                        ret_text.push_str(" ");
                    }
                    Event::FootnoteReference(name) => {
                        let len = self.numbers.len() + 1;
                        let number = *self.numbers.entry(name).or_insert(len);
                        ret_text.push_str(&format!("[{}]", number));
                    }
                    Event::TaskListMarker(true) => { ret_text.push_str("[x]")}
                    Event::TaskListMarker(false) => { ret_text.push_str("[ ]")}
                }
            }
            ret_text
        }
    }
}
