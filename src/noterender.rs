pub mod noterender {
    use std::collections::VecDeque;
    use std::sync::{Arc, Mutex};

    use hotwatch::Hotwatch;
    use web_view::*;

    use crate::incremental_dom::incremental_dom::push_incremental_dom;
    use crate::markdown_handler::markdown_handler::MarkdownHandler;
    use crate::PROJECT_DIR;

    pub enum JavascriptEvent {
        NONE,
        READY,
        TEST,
    }

    impl From<&str> for JavascriptEvent {
        fn from(arg: &str) -> Self {
            return match arg {
                "ready" => JavascriptEvent::READY,
                "test" => JavascriptEvent::TEST,
                _ => JavascriptEvent::NONE,
            };
        }
    }
    pub struct NoteRender {
        webview_handle: Option<Handle<JavascriptEvent>>,
        event_queue: Arc<Mutex<VecDeque<JavascriptEvent>>>,
        loaded_page: Arc<String>,
        file_watcher: Hotwatch,
        markdownhandler: Arc<Mutex<MarkdownHandler>>,
    }
    impl NoteRender {
        pub fn new() -> NoteRender {
            return NoteRender {
                webview_handle: None,
                event_queue: Arc::new(Mutex::new(VecDeque::new())),
                loaded_page: Arc::new("".to_string()),
                file_watcher: Hotwatch::new().expect("Couldn't instantiate file watcher"),
                markdownhandler: Arc::new(Mutex::new(MarkdownHandler::new(""))),
            };
        }

        pub fn load_page(&mut self, incremental_dom: String) {
            self.loaded_page = Arc::new(incremental_dom);
            self.load_html_into_webview();
        }
        pub fn get_markdown_handler(&self) -> Arc<Mutex<MarkdownHandler>> {
            self.markdownhandler.clone()
        }

        fn load_html_into_webview(&mut self) {
            let loaded_page = self.loaded_page.clone();
            match &self.webview_handle {
                Some(handle) => {
                    handle.dispatch(move |view| {
                        view.eval(&format!("doIncrementalDom(String.raw`{}`);", loaded_page))?;
                        view.eval("on_body_change()")?;
                        Ok(())
                    });
                }
                _ => {}
            };
        }

        pub fn build_webview<'a, 'b>(&'a mut self) -> WVResult<WebView<'b, JavascriptEvent>> {
            let event_queue_ref = self.event_queue.clone();
            let view = web_view::builder()
                .title("Note Renderer")
                .content(Content::Html(inject_resources(&self.loaded_page.clone())))
                .size(320, 480)
                .resizable(true)
                .user_data(JavascriptEvent::NONE)
                .invoke_handler(move |view, arg| {
                    let event = JavascriptEvent::from(arg);
                    event_queue_ref.lock().unwrap().push_front(event);
                    Ok(())
                })
                .build()?;
            self.webview_handle = Some(view.handle());

            Ok(view)
        }

        fn handle_events(&mut self) {
            let mutex_ref = self.event_queue.clone();
            let mutex_lock = mutex_ref.try_lock();
            match mutex_lock {
                Ok(mut event_queue) => {
                    while !event_queue.is_empty() {
                        match event_queue.pop_back() {
                            Some(event) => match event {
                                JavascriptEvent::READY => {
                                    println!("recieved the Ready event!");
                                    self.load_html_into_webview();
                                }
                                JavascriptEvent::TEST => {
                                    println!("recieved the test event!");
                                }
                                _ => {
                                    // Unhandled event
                                    return;
                                }
                            },
                            _ => {
                                // Event queue is empty
                                return;
                            }
                        }
                    }
                }
                _ => {
                    // Event queue is being written to, we'll get it the next time around
                    return;
                }
            }
        }
        fn update(&mut self) {
            let markdownhandler = self.get_markdown_handler();
            if let Ok(mut mutexguard) = markdownhandler.try_lock() {
                if mutexguard.do_refresh {
                    mutexguard.do_refresh = false;
                    let parser = mutexguard.gen_parser();
                    let mut incremental_dom_string = String::new();
                    push_incremental_dom(&mut incremental_dom_string, parser);
                    self.load_page(incremental_dom_string);
                }
            };
        }

        pub fn run(&mut self) -> WVResult<()> {
            loop {
                self.handle_events();
                self.update();
            }
        }
    }
    pub fn inject_resources(html: &str) -> String {
        let mut inject_string = "".to_string();
        for file in PROJECT_DIR
            .get_dir("css_inject")
            .expect("Couldn't access embedded css_inject resources.")
            .files()
        {
            if file.path().extension().is_some() && file.path().extension().unwrap() == "css" {
                inject_string.push_str(&format!(
                    "<link rel=\"stylesheet\" href=\"http://127.0.0.1:8080/{}\"></style>\n",
                    file.path().file_name().unwrap().to_str().unwrap()
                ));
            }
        }
        for file in PROJECT_DIR
            .get_dir("javascript_inject")
            .expect("Couldn't access embedded javascript_inject resources.")
            .files()
        {
            if file.path().extension().is_some() && file.path().extension().unwrap() == "js" {
                inject_string.push_str(&format!(
                    "<script type=\"text/javascript\" src=\"http://127.0.0.1:8080/{}\"></script>\n",
                    file.path().file_name().unwrap().to_str().unwrap()
                ));
            }
        }
        return format!(
            "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n{}\n</head>\n<body id=\"content\" class=\"markdown-body\" onload=\"on_ready()\">\n{}\n</body></html>",
            inject_string, html
        );
    }
}
