
use tauri::{AppHandle, GlobalWindowEvent, Manager, App, Error};

use crate::{content::{
    sync_content, sync_source
}, clipboard::store::{get_record, get_record_old, Clip}, setup::{NEW_CLIP}};
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
  println!("get newest data from js! {}", last_id);
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


#[tauri::command]
pub fn get_older(last_id: i16) -> Vec<Clip> {
  println!("get old data from js! {}", last_id);
  let mut clips = get_record_old(last_id);


  let empty: Vec<Clip> = Vec::new();
  clips = match clips {
    Ok(v) => return v,
    Err(err) => {
      println!("select error:{}", err);
      return empty;
    }
  };

  empty
}

#[tauri::command]
pub fn do_copy(id:i16) {
  println!("do copy event {}", id)
  
}
