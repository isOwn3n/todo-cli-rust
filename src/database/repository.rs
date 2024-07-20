use anyhow::Result as AnyHowResult;
use std::collections::HashMap;

use sqlite::{Connection, Result, State};

use super::queries::Todo;

pub fn create_todos_table(conn: &mut Connection) {
    let query = "
        CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title VARCHAR(255),
            description TEXT NULL,
            is_done INTEGER DEFAULT 0,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
    ";
    conn.execute(query).unwrap();
}

// pub fn update_todo(conn: &mut Connection, title: &str, description: &str) -> Result<()> {
//     let mut new_todo = Todo::update(&title, &description);
//     new_todo.save(conn)?;
//     Ok(())
// }

pub fn add_new_todo(conn: &mut Connection, title: &str, description: &str) -> Result<()> {
    let mut new_todo = Todo::new(&title, &description);
    new_todo.save(conn)?;
    Ok(())
}

pub fn done_todo(conn: &mut Connection, todo_id: u32) -> Result<()> {
    Todo::done(conn, todo_id)
}

pub fn get_todo(conn: &mut Connection, todo_id: u32) -> AnyHowResult<HashMap<String, String>> {
    let mut data = Todo::get(conn, todo_id)?;

    if let Ok(State::Row) = data.next() {
        let mut todo_data: HashMap<String, String> = HashMap::new();
        todo_data.insert(
            "id".to_string(),
            data.read::<String, _>("id").unwrap().to_string(),
        );
        todo_data.insert(
            "title".to_string(),
            data.read::<String, _>("title").unwrap().to_string(),
        );
        todo_data.insert(
            "description".to_string(),
            data.read::<String, _>("description").unwrap().to_string(),
        );
        todo_data.insert(
            "is_done".to_string(),
            (data.read::<i64, _>("is_done").unwrap() != 0).to_string(),
        );
        Ok(todo_data)
    } else {
        Err(anyhow::anyhow!("Todo not found"))
    }
}

pub fn get_all_todos(
    conn: &mut Connection,
    limit: u16,
    offset: u16,
) -> AnyHowResult<Vec<HashMap<String, String>>> {
    let mut data = Todo::all(conn, limit, offset)?;

    let mut todos_data: Vec<HashMap<String, String>> = Vec::new();
    while let Ok(State::Row) = data.next() {
        let mut todo_data = HashMap::new();

        todo_data.insert(
            "id".to_string(),
            data.read::<String, _>("id").unwrap().to_string(),
        );

        todo_data.insert(
            "title".to_string(),
            data.read::<String, _>("title").unwrap().to_string(),
        );

        todo_data.insert(
            "description".to_string(),
            data.read::<String, _>("description").unwrap().to_string(),
        );

        todo_data.insert(
            "is_done".to_string(),
            (data.read::<i64, _>("is_done").unwrap() != 0).to_string(),
        );

        todos_data.push(todo_data);
    }

    Ok(todos_data)
}

pub fn get_count(conn: &mut Connection) -> Result<u16> {
    let mut data = Todo::count(conn)?;
    if let Ok(State::Row) = data.next() {
        Ok(data
            .read::<String, _>("count")
            .unwrap()
            .as_str()
            .parse()
            .unwrap())
    } else {
        Ok(0)
    }
}

pub fn delete_a_todo(conn: &mut Connection, id: u32) -> Result<()> {
    Todo::delete(conn, id)?;
    Ok(())
}
