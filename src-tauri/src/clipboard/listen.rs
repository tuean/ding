use std::error::Error;

// use clipboard_master::CallbackResult;

use clipboard_rs::{
	Clipboard, ClipboardContext, ClipboardHandler, ClipboardWatcher, ClipboardWatcherContext, ContentFormat,
};
use std::{thread, time::Duration};
use tauri::AppHandle;

use crate::setup::broadcast_new_clipboard_event;
use crate::clipboard::store::add_record;

// use super::store::Clip;


// all clipboard type see: https://learn.microsoft.com/zh-cn/windows/win32/dataxchg/standard-clipboard-formats?redirectedfrom=MSDN


pub struct Manager {
    pub apphandle: AppHandle,
    pub clipboard_context: ClipboardContext
}

impl Manager {
    pub fn new(app_handle: AppHandle) -> Manager {
        Self {
            apphandle: app_handle,
            clipboard_context: ClipboardContext::new().unwrap()
        }
    }

    // pub fn set_copy(&self, info:Clip) -> Result<(), Error> {
    //     match info.content_type {
    //         super::store::ClipType::Text => {
    //             self.clipboardMonitor.set_text(info.content);
    //         },
    //         // super::store::ClipType::Image => todo!(),
    //         // super::store::ClipType::File => todo!(),
    //         // super::store::ClipType::Html => todo!(),
    //         // super::store::ClipType::Rtf => todo!(),
    //         // super::store::ClipType::Unknown => todo!(),
    //     }
    //     Ok(())
    // }
}


impl ClipboardHandler for Manager {
    fn on_clipboard_change(&mut self) {
        println!("receive change event");
        let types = self.clipboard_context.available_formats().unwrap();
	    println!("{:?}", types);
        let mut broadcast = false;

        let has_rtf = self.clipboard_context.has(ContentFormat::Rtf);
        if has_rtf && !broadcast {
            broadcast = true;
            println!("has_rtf={}", has_rtf);
            let rtf = self.clipboard_context.get_rich_text().unwrap_or("".to_string());
            let _ = add_record(&rtf, crate::clipboard::store::ClipType::Rtf);
            println!("rtf={}", rtf);
        }
        
    
        let has_html = self.clipboard_context.has(ContentFormat::Html);
        if has_html && !broadcast{
            broadcast = true;
            println!("has_html={}", has_html);
            let html = self.clipboard_context.get_html().unwrap_or("".to_string());
            let _ = add_record(&html, crate::clipboard::store::ClipType::Html);
            println!("html={}", html);
        }
    

        let has_file = self.clipboard_context.has(ContentFormat::Files);
        if has_file && !broadcast{
            broadcast = true;
            println!("has_file={}", has_rtf);
            let file = self.clipboard_context.get_text().unwrap_or("".to_string());
            // let _ = add_record(&file, crate::clipboard::store::ClipType::File);
            println!("file={}", file);
        }


        let has_image = self.clipboard_context.has(ContentFormat::Image);
        if has_image && !broadcast{
            broadcast = true;
            println!("has_image={}", has_rtf);
            let iamge = self.clipboard_context.get_text().unwrap_or("".to_string());
            // let _ = add_record(&file, crate::clipboard::store::ClipType::File);
            println!("file={}", iamge);
        }

        let has_text = self.clipboard_context.has(ContentFormat::Text);
        if has_text && !broadcast{
            broadcast = true;
            println!("has_text={}", has_rtf);
            let text = self.clipboard_context.get_text().unwrap_or("".to_string());
            let _ = add_record(&text, crate::clipboard::store::ClipType::Text);
            println!("text={}", text);
        }

        // let others = self.clipboard_context.has(ContentFormat::Other);
        // if others {
        //     broadcast = true;
        //     println!("others={}", has_rtf);
        //     let others = self.clipboard_context.get_text().unwrap_or("".to_string());
        //     // let _ = add_record(&file, crate::clipboard::store::ClipType::File);
        //     println!("others={}", others);
        // }
    

        if broadcast {
            let _ = broadcast_new_clipboard_event(&self.apphandle);
        }
        
    }
}



 