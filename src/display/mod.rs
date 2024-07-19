use crossterm::{
    cursor::{self, Hide, Show},
    event::{self, Event, KeyCode, KeyEvent},
    ExecutableCommand,
};
use draw::{draw, handle_pressed_keys};
use std::{
    collections::HashMap,
    io::{self, Stdout, Write},
    time::Duration,
};

use crate::database::Database;

pub mod components;
pub mod draw;

#[derive(Debug, PartialEq)]
pub enum DisplayLocation {
    Main,
    Todo,
    Add,
    Quit,
}

pub struct Display {
    maxc: u16,
    maxl: u16,
    current_todo: u16,
    display: DisplayLocation,
    // todos_len: u16,
    todos_total_len: u16,
    todos: Vec<HashMap<String, String>>,
    todo_item: HashMap<String, String>,
    title: String,
    description: String,
}

impl Display {
    pub fn new(
        maxc: u16,
        maxl: u16,
        // todos_len: &mut u16,
        todos_total_len: &mut u16,
        todos: Vec<HashMap<String, String>>,
    ) -> Display {
        Display {
            maxc,
            maxl,
            current_todo: 2,
            display: DisplayLocation::Main,
            // todos_len: *todos_len,
            todos_total_len: *todos_total_len,
            todos,
            todo_item: HashMap::new(),
            title: String::new(),
            description: String::new(),
        }
    }

    pub fn display_holder(&mut self, stdout: &mut Stdout, database: &mut Database) -> Result<(), std::io::Error> {
        while self.display != DisplayLocation::Quit {
            handle_pressed_keys(self, database);
            // match self.display {
            //     DisplayLocation::Todo => {}
            //     _ => {}
            // }
            stdout.flush()?
        }
        Ok(())
    }
}

enum Field {
    Title,
    Description,
}

pub fn read_text_boxes(display: &mut Display) -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut title = String::new();
    let mut description = String::new();
    let mut current_field = Field::Title;

    stdout.execute(cursor::MoveTo(0, 0))?;
    stdout.flush()?;
    stdout.execute(Show)?;

    loop {
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(KeyEvent { code, .. }) => {
                    match code {
                        KeyCode::Esc => {
                            // Exit the loop
                            break;
                        }
                        KeyCode::Tab => {
                            // Switch between text boxes
                            *&mut current_field = if let Field::Title = current_field {
                                Field::Description
                            } else {
                                Field::Title
                            };
                            update_screen(display, title.clone(), description.clone());
                        }
                        // KeyCode::Enter => {
                        //     // Handle Enter key (optional)
                        // }
                        KeyCode::Backspace => {
                            // Handle Backspace key
                            if let Field::Title = current_field {
                                title.pop();
                            } else {
                                description.pop();
                            }
                            update_screen(display, title.clone(), description.clone());
                        }
                        KeyCode::Char(c) => {
                            // Handle character input
                            if let Field::Title = current_field {
                                title.push(c);
                            } else {
                                description.push(c);
                            }
                            update_screen(display, title.clone(), description.clone());
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
    stdout.execute(Hide)?;
    Ok(())
}

fn update_screen(display: &mut Display, title: String, description: String) {
    display.title = title.clone();
    display.description = description.clone();
    let _ = draw(display, components::add_todo_component);
}
