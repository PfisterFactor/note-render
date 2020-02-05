extern crate include_dir;
extern crate web_view;
extern crate neovim_lib;
extern crate pulldown_cmark;
extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate hotwatch;
extern crate simple_server;

mod noterender;
mod neovim_handler;
mod markdown_handler;
mod filesystem_handler;
use std::*;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use include_dir::*;
use web_view::*;

use crate::noterender::noterender::{inject_resources, JavascriptEvent, NoteRender};
use crate::markdown_handler::markdown_handler::MarkdownHandler;
use crate::neovim_handler::neovim_handler::NeovimHandler;
use crate::filesystem_handler::filesystem_handler::FilesystemHandler;
use hotwatch::Hotwatch;
use std::path::{Path, PathBuf};
use std::time::Duration;

pub static PROJECT_DIR: Dir = include_dir!("./local_resources");

fn main() {
    let file_path = env::args().nth(1).map(|it| PathBuf::from(it));
    if !FilesystemHandler::verify_file_argument(file_path.clone()) {
        eprintln!("Please pass a valid .mdl file path as an argument");
        return;
    }
    let file_path = file_path.unwrap();
    println!("Opening note_render window...");
    let mut noterender = NoteRender::new();
    let view = noterender.build_webview();
    println!("Establishing connection to neovim...");
    let neovim = NeovimHandler::try_new();
    println!("Creating filesystem handler and spawning local resource server...");
    let fs_handler = FilesystemHandler::new(file_path.clone(), noterender.get_markdown_handler());
    fs_handler.spawn_resource_server();
    println!("Loading {:#?} into window...", file_path);
    let file = fs::read_to_string(&file_path).unwrap();
    noterender.get_markdown_handler().lock().expect("Couldn't get initial lock on markdown handler").load_markdown(&file);

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
