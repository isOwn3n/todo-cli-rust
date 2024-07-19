use anyhow::Result as AnyHowResult;
use std::collections::HashMap;

use sqlite::{Connection, Result, State, Statement};

struct Todo {
    id: u32,
    title: String,
    description: String,
    is_done: bool,
    created_at: String,
    updated_at: String,
}

// struct FilterTodoArgs {
//     id: Option<u32>,
//     title: Option<String>,
//     description: Option<String>,
//     is_done: Option<bool>,
// }

impl Todo {
    // fn new(title: &str, description: &str) -> Self {
    //     Self {
    //         id: 0,
    //         title: title.to_string(),
    //         description: description.to_string(),
    //         is_done: false,
    //         created_at: String::new(),
    //         updated_at: String::new(),
    //     }
    // }

    // fn update(id: u32, title: &str, description: &str, is_done: bool) -> Self {
    //     Self {
    //         id,
    //         title: title.to_string(),
    //         description: description.to_string(),
    //         is_done,
    //         created_at: String::new(),
    //         updated_at: String::new(),
    //     }
    // }

    // fn save(&mut self, conn: &mut Connection) -> Result<()> {
    //     let mut statement;
    //     if self.id == 0 {
    //         let query = "
    //             INSERT INTO todos
    //                 (title, description, is_done, created_at, updated_at)
    //                 VALUES (?1, ?2, ?3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
    //         ";
    //         statement = conn.prepare(query)?;
    //         statement.bind((1, self.title.as_str()))?;
    //         statement.bind((2, self.description.as_str()))?;
    //         statement.bind((3, (self.is_done as i32).to_string().as_str()))?;
    //     } else {
    //         statement = conn.prepare("UPDATE todos SET title = ?1, description = ?2, is_done = ?3, updated_at = CURRENT_TIMESTAMP WHERE id = ?4")?;
    //         statement.bind((1, self.title.as_str()))?;
    //         statement.bind((2, self.description.as_str()))?;
    //         statement.bind((3, (self.is_done as i32).to_string().as_str()))?;
    //         statement.bind((4, self.id.to_string().as_str()))?;
    //     }

    //     statement.next()?;
    //     Ok(())
    // }

    // fn done(conn: &mut Connection, id: u32) -> Result<()> {
    //     let mut statement = conn.prepare(
    //         "UPDATE todos SET
    //         is_done = ?1,
    //         updated_at = CURRENT_TIMESTAMP
    //         WHERE id = ?2",
    //     )?;
    //     statement.bind((1, (true as i32).to_string().as_str()))?;
    //     statement.bind((2, id.to_string().as_str()))?;
    //     statement.next()?;
    //     Ok(())
    // }

    // fn get<'a>(conn: &'a mut Connection, id: u32) -> Result<Statement<'a>> {
    //     let query = "SELECT * FROM todos WHERE id = ?";
    //     let mut statement = conn.prepare(query)?;
    //     let _ = statement.bind((1, id.to_string().as_str()));
    //     Ok(statement)
    // }

    fn all<'a>(conn: &'a mut Connection, limit: u16, offset: u16) -> Result<Statement<'a>> {
        let query = "SELECT * FROM todos ORDER BY id DESC LIMIT ? OFFSET ?";
        let mut statement = conn.prepare(query)?;
        let _ = statement.bind((1, limit.to_string().as_str()));
        let _ = statement.bind((2, offset.to_string().as_str()));
        Ok(statement)
    }

    fn count<'a>(conn: &'a mut Connection) -> Result<Statement<'a>> {
        let query = "SELECT COUNT(*) as count FROM todos";
        let statement = conn.prepare(query)?;
        Ok(statement)
    }
}

// pub fn create_todos_table(conn: &mut Connection) {
//     let query = "
//         CREATE TABLE IF NOT EXISTS todos (
//             id INTEGER PRIMARY KEY AUTOINCREMENT,
//             title VARCHAR(255),
//             description TEXT NULL,
//             is_done INTEGER DEFAULT 0,
//             created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
//             updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
//         );
//     ";
//     conn.execute(query).unwrap();
// }

// pub fn add_new_todo(conn: &mut Connection, title: &str, description: &str) -> Result<()> {
//     let mut new_todo = Todo::new(&title, &description);
//     new_todo.save(conn)?;
//     Ok(())
// }

// pub fn done_todo(conn: &mut Connection, todo_id: u32) -> Result<()> {
//     Todo::done(conn, todo_id)
// }

// pub fn get_todo(conn: &mut Connection, todo_id: u32) -> AnyHowResult<HashMap<String, String>> {
//     let mut data = Todo::get(conn, todo_id)?;

//     if let Ok(State::Row) = data.next() {
//         let mut todo_data: HashMap<String, String> = HashMap::new();
//         todo_data.insert(
//             "id".to_string(),
//             data.read::<String, _>("id").unwrap().to_string(),
//         );
//         todo_data.insert(
//             "title".to_string(),
//             data.read::<String, _>("title").unwrap().to_string(),
//         );
//         todo_data.insert(
//             "description".to_string(),
//             data.read::<String, _>("description").unwrap().to_string(),
//         );
//         todo_data.insert(
//             "is_done".to_string(),
//             (data.read::<i64, _>("is_done").unwrap() != 0).to_string(),
//         );
//         Ok(todo_data)
//     } else {
//         Err(anyhow::anyhow!("Todo not found"))
//     }
// }

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
