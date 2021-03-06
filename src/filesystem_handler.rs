pub mod filesystem_handler {
    use crate::markdown_handler::markdown_handler::MarkdownHandler;
    use hotwatch::Hotwatch;
    use simple_server::*;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    pub struct FilesystemHandler {
        watching_path: Arc<Mutex<PathBuf>>,
        hotwatch: Hotwatch,
        local_resource_server: Arc<Server>,
        markdown_handler: Arc<Mutex<MarkdownHandler>>
    }
    impl FilesystemHandler {
        pub fn new(watch_path: &Path, markdown_handler: Arc<Mutex<MarkdownHandler>>) -> FilesystemHandler {
            let mut file_watcher = Hotwatch::new_with_custom_delay(Duration::new(0, 0))
                .expect("Couldn't create file watcher");
            let file_path = Arc::new(Mutex::new(watch_path.to_path_buf()));
            let file_path_ref = file_path.clone();
            let server = Server::new(move |request, mut response| {
                println!("Request received for {:#?}", request.uri().path());
                let request_path = Path::new(request.uri().path()).to_path_buf();
                let file_path = file_path_ref.lock().unwrap();
                let file_name = file_path.file_stem().unwrap();
                if let Some(ext) = request_path.extension() {
                    let ext = ext.to_str().unwrap();
                    if ext == "css" {
                        let embedded_path = format!(
                            "css_inject/{}",
                            request_path.file_name().unwrap().to_str().unwrap()
                        );
                        match crate::PROJECT_DIR.get_file(format!(
                            "css_inject/{}",
                            request_path.file_name().unwrap().to_str().unwrap()
                        )) {
                            Some(file) => {
                                return Ok(response.body(file.contents().to_vec())?);
                            }
                            None => {
                                response.status(StatusCode::NOT_FOUND);
                                return Ok(response.body("<p>404</p>".as_bytes().to_vec())?);
                            }
                        }
                    } else if ext == "js" {
                        let embedded_path = format!(
                            "javascript_inject/{}",
                            request_path.file_name().unwrap().to_str().unwrap()
                        );
                        match crate::PROJECT_DIR.get_file(embedded_path) {
                            Some(file) => {
                                return Ok(response.body(file.contents().to_vec())?);
                            }
                            None => {
                                response.status(StatusCode::NOT_FOUND);
                                return Ok(response.body("<p>404</p>".as_bytes().to_vec())?);
                            }
                        }
                    } else if ext == "ttf" || ext == "woff" || ext == "woff2" {
                        let embedded_path = format!(
                            "font_inject/{}",
                            request_path.file_name().unwrap().to_str().unwrap()
                        );
                        match crate::PROJECT_DIR.get_file(embedded_path) {
                            Some(file) => {
                                return Ok(response.body(file.contents().to_vec())?);
                            }
                            None => {
                                response.status(StatusCode::NOT_FOUND);
                                return Ok(response.body("<p>404</p>".as_bytes().to_vec())?);
                            }
                        }
                    }
                }
                let image_path = file_path.parent().unwrap().join(format!(
                    "images/{}{}",
                    file_name.to_str().unwrap(),
                    request_path.to_str().unwrap()
                ));
                let image_path = image_path.canonicalize();
                let response_contents = image_path.and_then(|path| fs::read(path));
                if let Ok(bytes) = response_contents {
                    Ok(response.body(bytes)?)
                } else {
                    response.status(StatusCode::NOT_FOUND);
                    Ok(response.body("<p>404</p>".as_bytes().to_vec())?)
                }
            });
            let mut fs_handler = FilesystemHandler {
                watching_path: file_path,
                hotwatch: file_watcher,
                local_resource_server: Arc::new(server),
                markdown_handler
            };
            fs_handler.watch_new_file(watch_path);
            fs_handler
        }
        pub fn watch_new_file(&mut self, path: &Path) {
            self.hotwatch
                .unwatch(self.watching_path.lock().unwrap().to_path_buf());
            *self.watching_path.lock().unwrap() = path.to_path_buf();
            let borrowed = self.markdown_handler.clone();
            self.hotwatch.watch(path, move |event| {
                match event {
                    hotwatch::Event::Write(file) => {
                        //dbg!("Write event received");
                        if let Ok(string_contents) = fs::read_to_string(&file) {
                            borrowed
                                .lock()
                                .expect("Couldn't get lock on markdown handler")
                                .load_markdown(&string_contents);
                        }
                    }
                    _ => {}
                }
            });
        }
        pub fn spawn_resource_server(&self) {
            let server = self.local_resource_server.clone();
            thread::spawn(move || {
                server.listen("127.0.0.1", "8080");
            });
        }
        pub fn verify_file_argument(file_path: Option<PathBuf>) -> bool {
            if let None = file_path {
                return false;
            }
            let file_path = file_path.unwrap();
            if file_path.extension().is_none()
                || (file_path.extension().unwrap() != "mdl" && file_path.extension().unwrap() != "md")
                || !file_path.exists()
            {
                return false;
            }
            return true;
        }
    }
}
