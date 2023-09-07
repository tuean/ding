
// use https://crates.io/crates/clipboard-master to listen change event
// use https://crates.io/crates/arboard to get content/file in clipboard
// todo: file type not support yet

mod store;
use clipboard_master::{Master, ClipboardHandler, CallbackResult};
use arboard::Clipboard;

use std::io;

use crate::clipboard::store::init_table;

struct Handler;

impl ClipboardHandler for Handler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        println!("Clipboard change happened!");
        listen();
        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        eprintln!("Error: {}", error);
        CallbackResult::Next
    }
}

fn listen() {
    let mut clipboard = Clipboard::new().unwrap();
    println!("Clipboard text was: {}", clipboard.get_text().unwrap());
}

pub fn clipboard_listen() {
    let _ = init_table();
    println!("start to listen clipboard");
    let _ = Master::new(Handler).run();
}