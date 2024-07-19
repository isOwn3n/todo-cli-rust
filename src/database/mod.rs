use std::collections::HashMap;

use sqlite::{Connection, Result};

pub mod queries;

pub struct Database {
    connection: Connection,
}

impl Database {
    fn new(db_name: &str) -> Self {
        Database {
            connection: Connection::open(db_name).unwrap(),
        }
    }
}

pub fn get_data(maxl: u16) -> Vec<HashMap<String, String>> {
    let mut conn = Database::new("db.sqlite");
    queries::get_all_todos(&mut conn.connection, (maxl / 2) - 1, 0).unwrap()
}

pub fn get_count() -> Result<u16> {
    let mut conn = Database::new("db.sqlite");
    Ok(queries::get_count(&mut conn.connection)?)
}

// pub fn get_count<'a>(conn: &'a mut Connection) -> Result<Statement<'a>> {
//     let mut conn = Database::new("db.sqlite");
//     let mut data = queries::Todo::count(conn)?;
//     if let Ok(State::Row) = data.next() {
//         Ok(data
//             .read::<String, _>("count")
//             .unwrap()
//             .as_str()
//             .parse()
//             .unwrap())
//     } else {
//         Ok(0)
//     }
// }
