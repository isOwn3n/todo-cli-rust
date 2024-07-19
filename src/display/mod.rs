use std::{
    collections::HashMap,
    io::{Stdout, Write},
};

use draw::handle_pressed_keys;

pub mod draw;

#[derive(Debug, PartialEq)]
pub enum DisplayLocation {
    Main,
    // Todo,
    Quit,
}

pub struct Display {
    maxc: u16,
    maxl: u16,
    current_todo: u16,
    display: DisplayLocation,
    todos_len: u16,
    todos_total_len: u16,
    todos: Vec<HashMap<String, String>>,
}

impl Display {
    pub fn new(
        maxc: u16,
        maxl: u16,
        todos_len: &mut u16,
        todos_total_len: &mut u16,
        todos: Vec<HashMap<String, String>>,
    ) -> Display {
        Display {
            maxc,
            maxl,
            current_todo: 2,
            display: DisplayLocation::Main,
            todos_len: *todos_len,
            todos_total_len: *todos_total_len,
            todos,
        }
    }

    pub fn display_holder(&mut self, stdout: &mut Stdout) -> Result<(), std::io::Error> {
        while self.display != DisplayLocation::Quit {
            handle_pressed_keys(self);
            // match self.display {
            //     DisplayLocation::Todo => {}
            //     _ => {}
            // }
            stdout.flush()?
        }
        Ok(())
    }
}
