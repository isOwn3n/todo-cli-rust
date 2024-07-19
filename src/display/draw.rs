use crossterm::{
    cursor::{self},
    event::{poll, read, Event, KeyCode},
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};

use std::{
    io::{self, Stdout, Write},
    thread::sleep,
    time::Duration,
};

use crate::database::{self, Database};

use super::{
    components::{self, main_components, todo_component},
    read_text_boxes, Display, DisplayLocation,
};

pub fn handle_pressed_keys(display: &mut Display, database: &mut Database) {
    if poll(Duration::from_millis(10)).unwrap() {
        let key = read().unwrap();

        while poll(Duration::from_millis(0)).unwrap() {
            let _ = read();
        }

        match key {
            Event::Key(event) => match event.code {
                KeyCode::Down => {
                    if display.display == DisplayLocation::Main {
                        if (display.todos_total_len * 2) - 1 < display.current_todo {
                            display.current_todo = 2;
                        } else {
                            display.current_todo += 2;
                        }
                        let _ = draw(display, main_components);
                    } else if display.display == DisplayLocation::Todo {
                        let _ = draw(display, todo_component);
                    }
                }
                KeyCode::Up => {
                    if display.display == DisplayLocation::Main {
                        if (display.todos_total_len * 2) - 1 == display.current_todo {
                            display.current_todo -= 2;
                        } else if 2 >= display.current_todo {
                            display.current_todo = display.todos_total_len * 2;
                        } else {
                            display.current_todo -= 2;
                        }
                        let _ = draw(display, main_components);
                    } else if display.display == DisplayLocation::Todo {
                        let _ = draw(display, todo_component);
                    }
                }
                KeyCode::Char('q') => display.display = DisplayLocation::Quit,
                KeyCode::Char('r') => {
                    display.todos = database::get_data(database, display.maxl);
                    clearscreen::clear().expect("");
                    display.current_todo = 2;
                    sleep(Duration::from_millis(10));
                    let _ = draw(display, components::main_components);
                }
                KeyCode::Char(' ') => {
                    let todo_id = &display.todos[(display.current_todo / 2) as usize - 1]["id"];
                    let _ = database::done(database, todo_id.as_str().parse().unwrap());
                    display.todos = database::get_data(database, display.maxl);
                    let _ = draw(display, components::main_components);
                }
                KeyCode::Enter => {
                    display.display = DisplayLocation::Todo;
                    let id = &display.todos[(display.current_todo / 2) as usize - 1]["id"];
                    display.todo_item = database::get_one_todo(database, id.parse().unwrap());
                    let _ = draw(display, components::todo_component);
                }
                KeyCode::Backspace => {
                    if display.display != DisplayLocation::Main {
                        display.display = DisplayLocation::Main;
                        let _ = draw(display, components::main_components);
                    }
                }
                KeyCode::Char('a') => {
                    display.display = DisplayLocation::Add;
                    let _ = draw(display, components::add_todo_component);

                    let _ = read_text_boxes(display);
                    // let _ = new_todo("title".to_owned(), "description".to_owned());
                }
                _ => {}
            },
            _ => {}
        }
    }
}

pub fn draw(
    display: &mut Display,
    components: fn(
        display: &mut Display,
        stdout: &mut Stdout,
        y: u16,
        z: &mut u16,
    ) -> io::Result<()>,
) -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    let mut z: u16 = 0;

    for y in 0..display.maxl {
        for x in 0..display.maxc {
            if (y == 0 || y == display.maxl - 1) || (x == 0 || x == display.maxc - 1) {
                let _ = components(display, &mut stdout, y, &mut z);

                if y == 0 && x == 0 {
                    stdout
                        .queue(cursor::MoveTo(x, y))?
                        .queue(style::PrintStyledContent("╔".magenta()))?;
                } else if y == 0 && x == display.maxc - 1 {
                    stdout
                        .queue(cursor::MoveTo(x, y))?
                        .queue(style::PrintStyledContent("╗".magenta()))?;
                } else if y == display.maxl - 1 && x == 0 {
                    stdout
                        .queue(cursor::MoveTo(x, y))?
                        .queue(style::PrintStyledContent("╚".magenta()))?;
                } else if y == display.maxl - 1 && x == display.maxc - 1 {
                    stdout
                        .queue(cursor::MoveTo(x, y))?
                        .queue(style::PrintStyledContent("╝".magenta()))?;
                } else if y == display.maxl - 1 || y == 0 {
                    stdout
                        .queue(cursor::MoveTo(x, y))?
                        .queue(style::PrintStyledContent("═".magenta()))?;
                } else {
                    stdout
                        .queue(cursor::MoveTo(x, y))?
                        .queue(style::PrintStyledContent("║".magenta()))?;
                }
            }
        }
    }
    stdout.flush()?;
    Ok(())
}
