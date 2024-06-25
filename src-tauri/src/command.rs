// use tauri::{App, AppHandle, Error, GlobalWindowEvent, Manager};

use std::cmp::PartialEq;
use crate::{
    clipboard::{
        store::{get_clip_by_id, get_record, get_record_old, Clip}
    },
    content::{sync_content, sync_source}, keyboard::paste,
};
use clipboard_rs::{
    Clipboard, ClipboardContext
};

use std::thread;
use std::time::Duration;
use crate::clipboard::store::ClipType::Rtf;

#[tauri::command]
pub fn sync_html(content: String) {
    println!("invoked from JS! {}", content);
    sync_content(content)
}

#[tauri::command]
pub fn sync_md(content: String) {
    println!("invoked from JS! {}", content);
    sync_source(content)
}



#[tauri::command]
pub fn get_clipboard(last_id: i16) -> Vec<Clip> {
    println!("get newest data from js! {}", last_id);
    let clips = get_record(last_id);
    let empty: Vec<Clip> = Vec::new();
    match clips {
        Ok(v) => return v,
        Err(err) => {
            println!("select error:{}", err);
            return empty;
        }
    };
}

#[tauri::command]
pub fn get_older(last_id: i16) -> Vec<Clip> {
    println!("get old data from js! {}", last_id);
    let clips = get_record_old(last_id);

    let empty: Vec<Clip> = Vec::new();
    match clips {
        Ok(v) => return v,
        Err(err) => {
            println!("select error:{}", err);
            return empty;
        }
    };
}


#[tauri::command]
pub fn do_paste(id: i16, window: tauri::Window) -> bool {
    println!("do paste event");
    let mut set_flag:bool = false;
    match get_clip_by_id(id) {
        Ok(data) => {
            let clipboard_context:ClipboardContext = ClipboardContext::new().unwrap();
            match data.content_type {
                crate::clipboard::store::ClipType::Text => {
                    let text = data.content;
                    match clipboard_context.set_text(text.clone()) {
                        Ok(_) => {
                            println!("set text done: {:?}", text.clone());
                            set_flag = true;
                        },
                        Err(_) => {
                            println!("set text err: {:?}", text.clone());
                        }
                    } ;
                },
                crate::clipboard::store::ClipType::Image => {
                    set_flag = true;
                },
                crate::clipboard::store::ClipType::File => {
                    set_flag = true;
                },
                crate::clipboard::store::ClipType::Html => {
                    set_flag = true;
                },
                crate::clipboard::store::ClipType::Rtf => {
                    set_flag = true;
                },
                crate::clipboard::store::ClipType::Unknown => {
                    set_flag = true;
                },
            }
        },
        Err(_) => {
            println!("get clio error: {}", id);
            return false;
        }
    };

    if set_flag {
        window.hide().unwrap();
        thread::sleep(Duration::from_micros(100));
        paste();
        return true;
    }
    return false;
}