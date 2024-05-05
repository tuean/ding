use std::error::Error;

use clipboard_master::CallbackResult;
use clipboard_rs::Clipboard;
use clipboard_rs::ClipboardHandler;
use clipboard_rs::ClipboardContext;
use tauri::{ AppHandle };

use crate::setup::broadcast_new_clipboard_event;

use super::store::Clip;


// all clipboard type see: https://learn.microsoft.com/zh-cn/windows/win32/dataxchg/standard-clipboard-formats?redirectedfrom=MSDN


struct ClipboardMonitor {
    pub apphandle: AppHandle,
    clipboardMonitor: ClipboardContext
}

impl ClipboardMonitor {
    fn new(app_handle: AppHandle) -> ClipboardMonitor {
        Self {
            apphandle: app_handle,
            clipboardMonitor: ClipboardContext::new().unwrap()
        }
    }

    pub fn set_copy(&self, info:Clip) -> Result<()> {
        match info.content_type {
            super::store::ClipType::Text => {
                self.clipboardMonitor.set_text(info.content);
            },
            super::store::ClipType::Image => {
                // self.clipboardMonitor.set_image(info.content);
            },
            super::store::ClipType::File => {
                // self.clipboardMonitor.set_files(info.content);
            },
            super::store::ClipType::Html => {
                // self.clipboardMonitor.set_files(info.content)?
            },
            super::store::ClipType::Rtf => todo!(),
            super::store::ClipType::Unknown => todo!(),
        }
        Ok(())
    }
}


impl ClipboardHandler for ClipboardMonitor {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        
        let _ = broadcast_new_clipboard_event(&self.apphandle);
    }
}



 