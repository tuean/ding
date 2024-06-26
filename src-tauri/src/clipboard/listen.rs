use std::error::Error;
use std::fs;
use std::path::PathBuf;

// use clipboard_master::CallbackResult;

use chrono::Local;
use clipboard_rs::common::{RustImage, RustImageBuffer};
use clipboard_rs::{
    Clipboard, ClipboardContext, ClipboardHandler, ClipboardWatcher, ClipboardWatcherContext,
    ContentFormat,
};

use log::info;
use tauri::api::path::data_dir;
use tauri::{AppHandle, App};

use crate::clipboard::store::add_record;
use crate::setup::broadcast_new_clipboard_event;

// use super::store::Clip;

// all clipboard type see: https://learn.microsoft.com/zh-cn/windows/win32/dataxchg/standard-clipboard-formats?redirectedfrom=MSDN

pub struct Manager {
    pub apphandle: AppHandle,
    pub clipboard_context: ClipboardContext,
}

impl Manager {
    pub fn new(app_handle: AppHandle) -> Manager {
        Self {
            apphandle: app_handle,
            clipboard_context: ClipboardContext::new().unwrap(),
        }
    }

}

impl ClipboardHandler for Manager {
    fn on_clipboard_change(&mut self) {
        info!("receive change event");
        let types = self.clipboard_context.available_formats().unwrap();
        info!("{:?}", types);
        let mut broadcast = false;

        let has_rtf = self.clipboard_context.has(ContentFormat::Rtf);
        if has_rtf && !broadcast {
            broadcast = true;
            info!("has_rtf={}", has_rtf);
            let rtf = self
                .clipboard_context
                .get_rich_text()
                .unwrap_or("".to_string());
            let _ = add_record(&rtf, crate::clipboard::store::ClipType::Rtf);
            info!("rtf={}", rtf);
        }

        let has_html = self.clipboard_context.has(ContentFormat::Html);
        if has_html && !broadcast {
            broadcast = true;
            info!("has_html={}", has_html);
            let html = self.clipboard_context.get_html().unwrap_or("".to_string());
            let _ = add_record(&html, crate::clipboard::store::ClipType::Html);
            info!("html={}", html);
        }

        let has_file = self.clipboard_context.has(ContentFormat::Files);
        if has_file && !broadcast {
            broadcast = true;
            info!("has_file={}", has_file);
            let file = self.clipboard_context.get_files().unwrap_or(Vec::new());
            let single_file_path_string = file
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>()
                .join(";");
            let _ = add_record(
                &single_file_path_string,
                crate::clipboard::store::ClipType::File,
            );
            info!("file={:?}", file);
        }

        let has_image = self.clipboard_context.has(ContentFormat::Image);
        if has_image && !broadcast {
            broadcast = true;
            info!("has_image={}", has_image);
            let rust_image_data = self.clipboard_context.get_image();
            match rust_image_data {
                Ok(img) => {
                    // let bmp = img.to_bitmap().unwrap();
                    // let bytes = bmp.get_bytes();
                    let mut data_path: std::path::PathBuf = data_dir().unwrap();
                    data_path.push("cache");
                    data_path.push("images");
                    let current_date: chrono::prelude::NaiveDate = Local::now().date_naive();
                    info!("当前日期： {}", current_date);
                    data_path.push(current_date.format("%Y-%m-%d").to_string());
                    if !data_path.exists() {
                        fs::create_dir_all(&data_path).unwrap()
                    }

                    let current_timestamp = Local::now().timestamp();
                    info!("当前Unix时间戳： {}", current_timestamp);
                    data_path.push(current_timestamp.to_string() + ".png");
                    let path = data_path.to_str().unwrap();
                    info!("path{:?}", path);
                    match img.save_to_path(&path) {
                        Ok(_) => {
                            info!("save image success");
                            let _ = add_record(&data_path.to_str().unwrap().to_string(), crate::clipboard::store::ClipType::Image);
                        },
                        Err(err) => print!("save error:{:?}", err)
                    }
                }
                Err(err) => {
                    info!("err={}", err)
                }
            }
        }

        let has_text = self.clipboard_context.has(ContentFormat::Text);
        if has_text && !broadcast {
            broadcast = true;
            info!("has_text={}", has_rtf);
            let text = self.clipboard_context.get_text().unwrap_or("".to_string());
            let _ = add_record(&text, crate::clipboard::store::ClipType::Text);
            info!("text={}", text);
        }

        // let others = self.clipboard_context.has(ContentFormat::Other);
        // if others {
        //     broadcast = true;
        //     info!("others={}", has_rtf);
        //     let others = self.clipboard_context.get_text().unwrap_or("".to_string());
        //     // let _ = add_record(&file, crate::clipboard::store::ClipType::File);
        //     info!("others={}", others);
        // }

        if broadcast {
            let _ = broadcast_new_clipboard_event(&self.apphandle);
        }
    }
}
