use rusqlite::{Connection, Result, Error};

#[derive(Debug)]
pub enum ClipType {
    Text,
    Image,
    File
}

#[derive(Debug)]
struct Clip {
    id: i32,
    content_type: ClipType,
    content: String,
    date: i32
}

pub fn init_table() -> Result<(), Error> {
    println!("init table");
    let conn: Connection = Connection::open("clipboard.db")?;
    // let conn = Connection::open_in_memory()?;
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

pub fn add_record(content:&String, clipType:ClipType) -> Result<(), Error> {
    let conn: Connection = Connection::open("clipboard.db")?;
    match conn.execute(
        "insert into tb_clipboard values (
            0, ?1, ?2, ?3
        )", ()
    ) {
        Ok(_) => { println!("add record success")},
        Err(e) => { println!("add record error: {}", e) },
    }
    Ok(())
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