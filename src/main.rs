extern crate hotwatch;
extern crate include_dir;
#[macro_use]
extern crate lazy_static;
extern crate pulldown_cmark;
extern crate regex;
extern crate simple_server;
extern crate web_view;

use std::*;
use std::path::PathBuf;

use include_dir::*;

use crate::filesystem_handler::filesystem_handler::FilesystemHandler;
use crate::noterender::noterender::{inject_resources, NoteRender};

mod filesystem_handler;
mod incremental_dom;
mod markdown_handler;
mod noterender;

pub static PROJECT_DIR: Dir = include_dir!("./local_resources");

fn main() {
    let file_path = env::args().nth(1).map(|it| fs::canonicalize(it).unwrap());
    if !FilesystemHandler::verify_file_argument(file_path.clone()) {
        eprintln!("Please pass a valid .mdl/.md file path as an argument");
        return;
    }
    let file_path = file_path.unwrap();
    println!("Opening note_render window...");
    let mut noterender = NoteRender::new(&file_path);
    let view = noterender.build_webview();
    println!("Creating filesystem handler and spawning local resource server...");
    noterender.get_filesystem_handler().lock().unwrap().spawn_resource_server();
    match view {
        Ok(view) => {
            thread::spawn(move || {
                noterender.run();
            });
            view.run();
        }
        Err(e) => {
            eprintln!("Could not build webview! Error: {}\nExiting...", e);
        }
    }
}
