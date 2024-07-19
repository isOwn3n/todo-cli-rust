use std::collections::HashMap;

// use repository::add_new_todo;
use sqlite::{Connection, Result};

pub mod queries;
pub mod repository;

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

pub fn init_database() -> Result<Database> {
    Ok(Database::new("db.sqlite"))
}

pub fn create_table(conn: &mut Database) -> Result<()> {
    repository::create_todos_table(&mut conn.connection);
    Ok(())
}

pub fn get_data(conn: &mut Database, maxl: u16) -> Vec<HashMap<String, String>> {
    repository::get_all_todos(&mut conn.connection, (maxl / 2) - 1, 0).unwrap()
}

pub fn get_count(conn: &mut Database) -> Result<u16> {
    Ok(repository::get_count(&mut conn.connection)?)
}

pub fn done(conn: &mut Database, id: u32) -> Result<()> {
    repository::done_todo(&mut conn.connection, id)
}

pub fn get_one_todo(conn: &mut Database, id: u32) -> HashMap<String, String> {
    repository::get_todo(&mut conn.connection, id).unwrap()
}

// pub fn new_todo(conn: &mut Database,title: String, description: String) -> Result<()> {
//     add_new_todo(&mut conn.connection, title.as_str(), description.as_str())
// }
