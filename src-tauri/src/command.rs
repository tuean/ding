// use tauri::{App, AppHandle, Error, GlobalWindowEvent, Manager};

use std::cmp::PartialEq;
use crate::{
    clipboard::{
        store::{get_clip_by_id, get_record, get_record_old, Clip}
    },
    content::{sync_content, sync_source}, keyboard::paste,
};
use clipboard_rs::{Clipboard, ClipboardContext, RustImageData};

use std::thread;
use std::time::Duration;
use clipboard_rs::common::RustImage;
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
                    let image_file_path = data.content;
                    let image_data = RustImageData::from_path(image_file_path.as_str());
                    if !image_data.is_ok() {
                        return false;
                    }
                    match clipboard_context.set_image(image_data.unwrap()) {
                        Ok(_) => {
                            println!("set image done: {:?}", image_file_path.clone());
                            set_flag = true;
                        },
                        Err(_) => {
                            println!("set image err: {:?}", image_file_path.clone());
                        }
                    }
                },
                crate::clipboard::store::ClipType::File => {
                    let file_path = data.content;
                    let v = file_path.clone();
                    match clipboard_context.set_files(vec![file_path]) {
                        Ok(_) => {
                            println!("set image done: {:?}", v.clone());
                            set_flag = true;
                        },
                        Err(_) => {
                            println!("set image err: {:?}", v.clone());
                        }
                    }
                },
                crate::clipboard::store::ClipType::Html => {
                    let html = data.content;
                    let v = html.clone();
                    match clipboard_context.set_html(html) {
                        Ok(_) => {
                            println!("set image done: {:?}", v.clone());
                            set_flag = true;
                        },
                        Err(_) => {
                            println!("set image err: {:?}", v.clone());
                        }
                    }
                },
                crate::clipboard::store::ClipType::Rtf => {
                    let rtf = data.content;
                    let v = rtf.clone();
                    match clipboard_context.set_rich_text(rtf) {
                        Ok(_) => {
                            println!("set image done: {:?}", v.clone());
                            set_flag = true;
                        },
                        Err(_) => {
                            println!("set image err: {:?}", v.clone());
                        }
                    }
                },
                crate::clipboard::store::ClipType::Unknown => {
                    // todo
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