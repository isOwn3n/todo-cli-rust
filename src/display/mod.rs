use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    terminal::size,
    ExecutableCommand
};
use draw::{draw, handle_pressed_keys};
use std::{
    collections::HashMap,
    io::{self, stdout, Stdout, Write},
    time::Duration,
};

use crate::database::{self, Database};

pub mod components;
pub mod draw;

#[derive(Hash, Eq, Debug, PartialEq)]
pub enum DisplayLocation {
    Main,
    Todo,
    Add,
    Quit,
}

pub type DisplayFunctionMap =
    HashMap<&'static DisplayLocation, fn(&mut Display, &mut Stdout, &mut u16) -> io::Result<()>>;

pub struct TextBox {
    title: String,
    description: String,
    display_title: String,
    display_description: String,
    current_field: Field,
    input_col_length: u16,
    title_current: usize,
    desc_current: usize,
}

impl TextBox {
    pub fn new() -> TextBox {
        TextBox {
            title: String::new(),
            description: String::new(),
            display_title: String::new(),
            display_description: String::new(),
            current_field: Field::Title,
            input_col_length: 0,
            title_current: 0,
            desc_current: 0,
        }
    }
    pub fn init(&mut self) {
        let title = &self.title;
        let desc = &self.description;
        let input_col_length = self.input_col_length as usize;

        if title.len() < self.input_col_length as usize {
            self.display_title = title.to_string();
        } else {
            let corrected_title_start = if self.title_current < input_col_length {
                0
            } else {
                self.title_current - input_col_length
            };
            self.display_title = title[corrected_title_start..self.title_current].to_string();
        }

        if desc.len() < self.input_col_length as usize {
            self.display_description = desc.to_string();
        } else {
            let corrected_desc_start = if self.desc_current < input_col_length {
                0
            } else {
                self.desc_current - input_col_length
            };
            self.display_description = desc[corrected_desc_start..self.desc_current].to_string();
        }
    }
}

pub struct Display<'a> {
    maxc: u16,
    pub maxl: u16,
    current_todo: u16,
    display: DisplayLocation,
    pub todos_len: u16,
    pub todos_total_len: u16,
    pub todos: Vec<HashMap<String, String>>,
    todo_item: HashMap<String, String>,
    functions_map: DisplayFunctionMap,
    screen: &'a mut Stdout,
    text: TextBox,
}

impl<'a> Display<'a> {
    pub fn new(
        maxc: u16,
        maxl: u16,
        todos_len: &mut u16,
        todos_total_len: &mut u16,
        todos: Vec<HashMap<String, String>>,
        functions: DisplayFunctionMap,
        sc: &'a mut Stdout,
    ) -> Display<'a> {
        Display {
            maxc,
            maxl,
            current_todo: 2,
            display: DisplayLocation::Main,
            todos_len: *todos_len,
            todos_total_len: *todos_total_len,
            todos,
            todo_item: HashMap::new(),
            functions_map: functions,
            screen: sc,
            text: TextBox::new(),
        }
    }

    pub fn display_holder(&mut self, database: &mut Database) -> Result<(), std::io::Error> {
        while self.display != DisplayLocation::Quit {
            handle_pressed_keys(self, database);
            // match self.display {
            //     DisplayLocation::Todo => {}
            //     _ => {}
            // }
            self.screen.flush()?
        }
        Ok(())
    }
}

#[derive(PartialEq)]
enum Field {
    Title,
    Description,
}

pub fn init_screen() -> io::Result<(Stdout, u16, u16, DisplayFunctionMap)> {
    let sc = stdout();
    let (maxc, maxl) = size().unwrap();
    // sc.execute(Hide)?;
    let mut functions: DisplayFunctionMap = HashMap::new();
    functions.insert(&DisplayLocation::Main, components::main_component);
    functions.insert(&DisplayLocation::Add, components::add_todo_component);
    functions.insert(&DisplayLocation::Todo, components::todo_component);
    Ok((sc, maxc, maxl, functions))
}

pub fn read_text_boxes(display: &mut Display, database: &mut Database) -> io::Result<()> {
    let mut title = String::new();
    let mut description = String::new();

    display.screen.execute(cursor::MoveTo(17, 2))?;
    display.screen.flush()?;
    display.screen.execute(cursor::Show)?;

    loop {
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(KeyEvent { code, .. }) => match code {
                    KeyCode::Esc => {
                        break;
                    }
                    KeyCode::Tab => {
                        display.text.current_field =
                            if let Field::Title = display.text.current_field {
                                Field::Description
                            } else {
                                Field::Title
                            };

                        update_screen(display, title.clone(), description.clone(), false);
                    }
                    KeyCode::Right => {
                        if display.text.current_field == Field::Title
                            && display.text.title_current < title.len()
                        {
                            display.text.title_current += 1;
                        } else if display.text.current_field == Field::Description
                            && display.text.desc_current < description.len()
                        {
                            display.text.desc_current += 1;
                        }
                        update_screen(display, title.clone(), description.clone(), true);
                    }
                    KeyCode::Left => {
                        if display.text.current_field == Field::Title
                            && display.text.title_current > (display.text.input_col_length).into()
                        {
                            display.text.title_current -= 1;
                        } else if display.text.current_field == Field::Description
                            && display.text.desc_current > (display.text.input_col_length).into()
                        {
                            display.text.desc_current -= 1;
                        }
                        update_screen(display, title.clone(), description.clone(), true);
                    }
                    KeyCode::Backspace => {
                        if let Field::Title = display.text.current_field {
                            if display.text.title_current > 0 {
                                title.remove(display.text.title_current - 1);
                                display.text.title_current -= 1;
                            }
                        } else {
                            if display.text.desc_current > 0 {
                                description.remove(display.text.desc_current - 1);
                                display.text.desc_current -= 1;
                            }
                        }
                        update_screen(display, title.clone(), description.clone(), false);
                    }
                    KeyCode::Char(c) => {
                        if let Field::Title = display.text.current_field {
                            title.insert(display.text.title_current, c);
                            display.text.title_current += 1;
                        } else {
                            description.insert(display.text.desc_current, c);
                            display.text.desc_current += 1;
                        }
                        update_screen(display, title.clone(), description.clone(), false);
                    }
                    KeyCode::Enter => {
                        database::new_todo(
                            database,
                            display.text.title.clone(),
                            display.text.description.clone(),
                        )
                        .unwrap();
                        break;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
    Ok(())
}

fn update_screen(display: &mut Display, title: String, description: String, is_shift: bool) {
    if !is_shift {
        display.text.title = title.clone();
        display.text.description = description.clone();
    }
    let _ = draw(display, components::add_todo_component);
}
