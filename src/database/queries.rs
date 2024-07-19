use anyhow::Result as AnyHowResult;
use std::collections::HashMap;

use sqlite::{Connection, Result, State, Statement};

pub struct Todo {
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
    pub fn new(title: &str, description: &str) -> Self {
        Self {
            id: 0,
            title: title.to_string(),
            description: description.to_string(),
            is_done: false,
            created_at: String::new(),
            updated_at: String::new(),
        }
    }

    pub fn update(id: u32, title: &str, description: &str, is_done: bool) -> Self {
        Self {
            id,
            title: title.to_string(),
            description: description.to_string(),
            is_done,
            created_at: String::new(),
            updated_at: String::new(),
        }
    }

    pub fn save(&mut self, conn: &mut Connection) -> Result<()> {
        let mut statement;
        if self.id == 0 {
            let query = "
                INSERT INTO todos
                    (title, description, is_done, created_at, updated_at)
                    VALUES (?1, ?2, ?3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
            ";
            statement = conn.prepare(query)?;
            statement.bind((1, self.title.as_str()))?;
            statement.bind((2, self.description.as_str()))?;
            statement.bind((3, (self.is_done as i32).to_string().as_str()))?;
        } else {
            statement = conn.prepare("UPDATE todos SET title = ?1, description = ?2, is_done = ?3, updated_at = CURRENT_TIMESTAMP WHERE id = ?4")?;
            statement.bind((1, self.title.as_str()))?;
            statement.bind((2, self.description.as_str()))?;
            statement.bind((3, (self.is_done as i32).to_string().as_str()))?;
            statement.bind((4, self.id.to_string().as_str()))?;
        }

        statement.next()?;
        Ok(())
    }

    pub fn done(conn: &mut Connection, id: u32) -> Result<()> {
        let query = "UPDATE todos
            SET 
            updated_at = CURRENT_TIMESTAMP,
            is_done = CASE
                WHEN is_done = 1 THEN 0
                ELSE 1
            END
            WHERE id = ?;";
        let mut statement = conn.prepare(query)?;
        statement.bind((1, id.to_string().as_str()))?;
        statement.next()?;
        Ok(())
    }

    pub fn get<'a>(conn: &'a mut Connection, id: u32) -> Result<Statement<'a>> {
        let query = "SELECT * FROM todos WHERE id = ?";
        let mut statement = conn.prepare(query)?;
        let _ = statement.bind((1, id.to_string().as_str()));
        Ok(statement)
    }

    pub fn all<'a>(conn: &'a mut Connection, limit: u16, offset: u16) -> Result<Statement<'a>> {
        let query = "SELECT * FROM todos ORDER BY id DESC LIMIT ? OFFSET ?";
        let mut statement = conn.prepare(query)?;
        let _ = statement.bind((1, limit.to_string().as_str()));
        let _ = statement.bind((2, offset.to_string().as_str()));
        Ok(statement)
    }

    pub fn count<'a>(conn: &'a mut Connection) -> Result<Statement<'a>> {
        let query = "SELECT COUNT(*) as count FROM todos";
        let statement = conn.prepare(query)?;
        Ok(statement)
    }
}
