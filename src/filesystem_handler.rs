pub mod filesystem_handler {
    use hotwatch::Hotwatch;
    use simple_server::*;
    use std::path::{PathBuf, Path};
    use crate::markdown_handler::markdown_handler::MarkdownHandler;
    use std::sync::{Mutex, Arc};
    use std::time::Duration;
    use std::fs;
    use std::thread;

    pub struct FilesystemHandler {
        watching_path: PathBuf,
        hotwatch: Hotwatch,
        local_resource_server: Arc<Server>,
        markdown_handler: Arc<Mutex<MarkdownHandler>>
    }
    impl FilesystemHandler {
        pub fn new(watch_path: PathBuf, markdownhandle: Arc<Mutex<MarkdownHandler>>) -> FilesystemHandler {
            let mut file_watcher = Hotwatch::new_with_custom_delay(Duration::new(0,0)).expect("Couldn't create file watcher");
            let borrowed = markdownhandle.clone();
            file_watcher.watch(&watch_path,  move |event| {
                match event {
                    hotwatch::Event::Write(file) => {
                        //dbg!("Write event received");
                        if let Ok(string_contents) = fs::read_to_string(&file) {
                            borrowed.lock().expect("Couldn't get lock on markdown handler").load_markdown(&string_contents);
                        }
                    },
                    _ => {}
                }
            });
            let file_path = watch_path.clone();
            let server = Server::new(move |request,mut response| {
                println!("Request received for {:#?}", request.uri().path());
                let request_path = Path::new(request.uri().path()).to_path_buf();
                let file_name = file_path.file_stem().unwrap();

                let image_path = file_path.parent().unwrap()
                    .join(format!("images/{}{}",file_name.to_str().unwrap(),request_path.to_str().unwrap()));
                let image_path = image_path.canonicalize();
                println!("Response would be {:#?}", image_path);
                let response_contents = image_path.and_then(|path| {
                    fs::read(path)
                });
                if let Ok(bytes) = response_contents {
                    Ok(response.body(bytes)?)
                }
                else {
                    response.status(StatusCode::NOT_FOUND);
                    Ok(response.body("<p>404</p>".as_bytes().to_vec())?)
                }
            });
            return FilesystemHandler {watching_path: watch_path, hotwatch: file_watcher, local_resource_server: Arc::new(server),markdown_handler: markdownhandle}
        }
        pub fn spawn_resource_server(&self) {
            let server = self.local_resource_server.clone();
            thread::spawn(move || {
                server.listen("127.0.0.1","8080");
            });
        }
        pub fn verify_file_argument(file_path: Option<PathBuf>) -> bool {
            if let None = file_path {
                return false;
            }
            let file_path = file_path.unwrap();
            if file_path.extension().is_none() || file_path.extension().unwrap() != "mdl" || !file_path.exists() {
                return false;
            }
            return true;
        }


    }
}