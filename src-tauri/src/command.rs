
use crate::{content::{
    sync_content, sync_source
}, clipboard::store::{get_record, Clip}};
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
pub fn get_clipboard(last_id: i16) -> Vec<Clip> {
  println!("invoked from JS! {}", last_id);
  let mut clips = get_record(last_id);
  let empty: Vec<Clip> = Vec::new();
  clips = match clips {
    Ok(v) => return v,
    Err(err) => {
      println!("select error:{}", err);
      return empty;
    }
  };
  // let result: String = serde_json::to_string(&clips).unwrap();
  // Some(result)
  // clips.unwrap()
}