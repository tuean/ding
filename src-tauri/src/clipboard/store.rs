use rusqlite::{Connection, Result, Error, params};
use std::{error::Error, result, time::{SystemTime, UNIX_EPOCH}};
use serde::Serialize;
use tauri::api::path::data_dir;

#[derive(Error, Debug)]
pub enum DBError {
    #[error("Database connection error")]
    ConnectionError(#[from] rusqlite::Error),
    #[error("Clip not found for the given ID")]
    ClipNotFoundError,
    // 可能还有其他错误类型...
}

#[derive(Debug, Serialize)]
pub enum ClipType {
    Text,
    Image,
    File,
    Unknown
}

impl ClipType {
    fn get_name(clip_type:ClipType) -> String {
        match clip_type {
            ClipType::Text => "text".to_owned(),
            ClipType::Image => "image".to_owned(),
            ClipType::File => "file".to_owned(),
            _ => "unknown".to_owned(),
        }
    }

    fn parse_name(name: String) -> Self {
        let r = match &name as &str {
            "text" => ClipType::Text,
            "image" => ClipType::Image,
            "file" => ClipType::File,
            _ => ClipType::Unknown
        };
        r
    }
}

#[derive(Debug, Serialize)]
#[warn(dead_code)]
pub struct Clip {
    id: i32,
    content_type: ClipType,
    content: String,
    date: i32
}

fn get_connection() -> Connection {
    let mut data_path: std::path::PathBuf = data_dir().unwrap();
    data_path.push("clipboard");
    let path = data_path.as_path().display().to_string();
    println!("database file path: {}", path);
    let conn: Connection = Connection::open(path).unwrap();
    conn
}

pub fn init_table() -> Result<(), Error> {
    println!("init table");
    let conn: Connection = get_connection();
    match conn.execute(
            "create table if not exists tb_clipboard (
                id INTEGER PRIMARY KEY,
                content_type varchar(10),
                content text,
                date INTEGER
            )", ()
        ) {
        Ok(_) => { println!("init table success")},
        Err(e) => { println!("init table error: {}", e) },
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
            println!("update exist {:?} date: {:?}", size, now);
            if size == 0 {
                match conn.execute(
                    "insert into tb_clipboard (content_type, content, date) values (
                        ?1, ?2, ?3
                    )", (c_type2, content, now)
                ) {
                    Ok(_) => { println!("add record success")},
                    Err(e) => { println!("add record error: {}", e) },
                }
            }

        },
        Err(e) => { println!("update exist error: {}", e)}
    } 
    
    
    Ok(())
}

pub fn get_record(last_id: i16) -> Result<Vec<Clip>, Error> {
    let conn: Connection = get_connection();
    let size:i8 = 20;
    let sql_with_param = "select id, content_type, content, date from tb_clipboard where id > ? order by id desc limit ?";
    let mut stmt = conn.prepare(sql_with_param)?;
    
    println!("sql_with_param: {} id: {}", sql_with_param, last_id);
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

pub fn get_record_old(last_id: i16) -> Result<Vec<Clip>, Error> {
    let conn: Connection = get_connection();
    let size:i8 = 20;

    let sql_with_param = "select id, content_type, content, date from tb_clipboard where id < ? order by id desc limit ?";
    let mut stmt = conn.prepare(sql_with_param)?;

    println!("sql_with_param: {} id: {}", sql_with_param, last_id);
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

pub fn get_clip_by_id(id: i16) -> Result<Clip, Error> {
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
        return Ok(result)
    } else {
        return Err(ClipNotFoundError)
    }
}

// #[derive(Debug)]
// struct Person {
//     id: i32,
//     name: String,
//     data: Option<Vec<u8>>,
// }

// fn main() -> Result<()> {
//     let conn = Connection::open_in_memory()?;

//     conn.execute(
//         "CREATE TABLE person (
//             id    INTEGER PRIMARY KEY,
//             name  TEXT NOT NULL,
//             data  BLOB
//         )",
//         (), // empty list of parameters.
//     )?;
//     let me = Person {
//         id: 0,
//         name: "Steven".to_string(),
//         data: None,
//     };
//     conn.execute(
//         "INSERT INTO person (name, data) VALUES (?1, ?2)",
//         (&me.name, &me.data),
//     )?;

//     let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
//     let person_iter = stmt.query_map([], |row| {
//         Ok(Person {
//             id: row.get(0)?,
//             name: row.get(1)?,
//             data: row.get(2)?,
//         })
//     })?;

//     for person in person_iter {
//         println!("Found person {:?}", person.unwrap());
//     }
//     Ok(())
// }