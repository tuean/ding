use rusqlite::{Connection, Result, Error, params};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::Serialize;
use tauri::api::path::data_dir;

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
    data_path.push("clipboard.db");
    let mut path = data_path.as_path().display().to_string();
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
    match conn.execute(
        "insert into tb_clipboard (content_type, content, date) values (
            ?1, ?2, ?3
        )", (ClipType::get_name(clip_type), content, now)
    ) {
        Ok(_) => { println!("add record success")},
        Err(e) => { println!("add record error: {}", e) },
    }
    Ok(())
}

pub fn get_record(last_id: i16) -> Result<Vec<Clip>, Error> {
    let conn: Connection = get_connection();
    let sql_with_param = "select id, content_type, content, date from tb_clipboard where id > ? limit 10";
    let mut stmt = conn.prepare(sql_with_param)?;
    
    println!("sql_with_param: {} id: {}", sql_with_param, last_id);
    let clips = stmt.query_map(params![&last_id], |row| {
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