
use crate::content::{
    sync_content, sync_source
};


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