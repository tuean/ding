
use crate::{content::{
    sync_content, sync_source
}, clipboard::store::{get_record}};
// use serde::Serialize;


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
pub fn get_clipboard(last_id: i16) -> Option<String>{
  println!("invoked from JS! {}", last_id);
  let clips = get_record(last_id).unwrap();
  let result = serde_json::to_string(&clips).unwrap();
  Some(result)
}