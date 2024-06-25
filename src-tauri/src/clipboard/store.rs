use log::{info, error};
use rusqlite::{Connection, Result, Error, params};
use std::{time::{SystemTime, UNIX_EPOCH}};
use serde::Serialize;
use crate::util::get_db_file_path;

#[derive(Debug, Serialize)]
pub enum ClipType {
    Text,
    Image,
    File,
    Html,
    Rtf,
    Unknown
}

impl ClipType {
    fn get_name(clip_type:ClipType) -> String {
        match clip_type {
            ClipType::Text => "text".to_owned(),
            ClipType::Image => "image".to_owned(),
            ClipType::File => "file".to_owned(),
            ClipType::Html => "html".to_owned(),
            ClipType::Rtf => "rtf".to_owned(),
            _ => "unknown".to_owned(),
        }
    }

    fn parse_name(name: String) -> Self {
        let r = match &name as &str {
            "text" => ClipType::Text,
            "image" => ClipType::Image,
            "file" => ClipType::File,
            "html" => ClipType::Html,
            "rtf" => ClipType::Rtf,
            _ => ClipType::Unknown
        };
        r
    }
}

#[derive(Debug, Serialize)]
#[warn(dead_code)]
pub struct Clip {
    pub id: i32,
    pub content_type: ClipType,
    pub content: String,
    pub date: i32
}

fn get_connection() -> Connection {
    let path = get_db_file_path();
    info!("database file path: {}", path);
    let conn: Connection = Connection::open(path).unwrap();
    conn
}

pub fn init_table() -> Result<(), Error> {
    info!("init table start");
    let conn: Connection = get_connection();
    match conn.execute(
            "create table if not exists tb_clipboard (
                id INTEGER PRIMARY KEY,
                content_type varchar(10),
                content text,
                date INTEGER
            )", ()
        ) {
        Ok(_) => { info!("init table success")},
        Err(e) => { error!("init table error: {}", e) },
    }
    Ok(())
}

pub fn add_record(content:&String, clip_type:ClipType) -> Result<(), Error> {
    let conn: Connection = get_connection();
    let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => 1,
    };

    let c_type: String = ClipType::get_name(clip_type);
    let c_type2 = c_type.clone();

    // 首先尝试将已有的结果往前推
    let update_exist = conn.execute("update tb_clipboard set date = ?1 where content_type = ?2 and content = ?3", 
    (now, c_type, content));

    match update_exist {
        Ok(size) => {
            info!("update exist {:?} date: {:?}", size, now);
            if size == 0 {
                match conn.execute(
                    "insert into tb_clipboard (content_type, content, date) values (
                        ?1, ?2, ?3
                    )", (c_type2, content, now)
                ) {
                    Ok(_) => { info!("add record success")},
                    Err(e) => { info!("add record error: {}", e) },
                }
            }

        },
        Err(e) => { info!("update exist error: {}", e)}
    } 
    
    
    Ok(())
}

pub fn get_record(last_id: i16) -> Result<Vec<Clip>, Error> {
    let conn: Connection = get_connection();
    let size:i8 = 20;
    let sql_with_param = "select id, content_type, content, date from tb_clipboard where id > ? order by id desc limit ?";
    let mut stmt = conn.prepare(sql_with_param)?;
    
    info!("sql_with_param: {} id: {}", sql_with_param, last_id);
    let clips = stmt.query_map(params![&last_id, &size], |row| {
        Ok(Clip {
             id: row.get(0)?, 
             content_type: ClipType::parse_name(row.get(1)?), 
             content: row.get(2)?, 
             date: row.get(3)? 
            })
    })?;
    let mut result = Vec::new();
    for c in clips {
        result.push(c?);
    }
    Ok(result)
}

pub fn get_record_old(last_id: i16) -> Result<Vec<Clip>> {
    let conn: Connection = get_connection();
    let size:i8 = 20;

    let sql_with_param = "select id, content_type, content, date from tb_clipboard where id < ? order by id desc limit ?";
    let mut stmt = conn.prepare(sql_with_param)?;

    info!("sql_with_param: {} id: {}", sql_with_param, last_id);
    let clips = stmt.query_map(params![&last_id, &size], |row| {
        Ok(Clip {
             id: row.get(0)?, 
             content_type: ClipType::parse_name(row.get(1)?), 
             content: row.get(2)?, 
             date: row.get(3)? 
            })
    })?;
    let mut result = Vec::new();
    for c in clips {
        result.push(c?);
    }
    Ok(result)
}

pub fn get_clip_by_id(id: i16) -> Result<Clip, rusqlite::Error> {
    let conn: Connection = get_connection();
    let sql_with_param = "select id, content_type, content, date from tb_clipboard where id = ?";
    let mut stmt = conn.prepare(sql_with_param)?;

    let mut clips = stmt.query_map(params![&id], |row| {
        Ok(Clip {
             id: row.get(0)?, 
             content_type: ClipType::parse_name(row.get(1)?), 
             content: row.get(2)?, 
             date: row.get(3)? 
            })
    })?;

    if let Some(result) = clips.next() {
        let result = result?;
        return Ok(result);
    } else {
        let custom_error = Error::SqliteFailure(rusqlite::ffi::Error {
            code: rusqlite::ffi::ErrorCode::Unknown, // 或选择其他合适的错误代码
            extended_code: 0, // 扩展错误码，根据需要设定
        }, Some(String::from("clip not found")));
        return Err(custom_error)
    }
    
}
