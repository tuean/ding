
// use https://crates.io/crates/clipboard-master to listen change event
// use https://crates.io/crates/arboard to get content/file in clipboard
// todo: file type not support yet

pub(crate) mod store;
use clipboard_master::{Master, ClipboardHandler, CallbackResult};
use arboard::Clipboard;
use tauri::{App, AppHandle};
use std::io;

// use crate::{clipboard::store::{init_table, add_record}, setup::broadcast_new_clipboard_event};
use crate::{clipboard::store::{add_record, init_table}, setup::broadcast_new_clipboard_event};

struct Handler {
    pub apphandle: AppHandle
}


impl ClipboardHandler for Handler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        println!("Clipboard change happened!");
        listen();
        let _ = broadcast_new_clipboard_event(&self.apphandle);
        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        eprintln!("Error: {}", error);
        CallbackResult::Next
    }
}

fn listen() {
    let mut clipboard = Clipboard::new().unwrap();
    match clipboard.get_text() {
        Ok(text) => {
            println!("Clipboard text was: {}", text);
            let _ = add_record(&text, store::ClipType::Text);
        },
        Err(_) => {},
    }
}

pub fn clipboard_listen(app: AppHandle) {
    let _ = init_table();
    println!("start to listen clipboard");
    let _ = Master::new(Handler{apphandle: app}).run();
}