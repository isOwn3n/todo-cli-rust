use crossterm::{
    cursor::{self, Show},
    event::{poll, read, Event, KeyCode},
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};

use std::{
    io::{self, Write},
    thread::sleep,
    time::Duration,
};

use crate::database::{self, initialize_all_data, Database};

use super::{components, read_text_boxes, Display, DisplayLocation};

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
                    }
                    let _ = draw(display, display.functions_map[&display.display]);
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
                    }
                    let _ = draw(display, display.functions_map[&display.display]);
                }
                KeyCode::Char('q') => display.display = DisplayLocation::Quit,
                KeyCode::Char('r') => {
                    initialize_all_data(database, display).unwrap();
                    clearscreen::clear().expect("");
                    display.current_todo = 2;
                    sleep(Duration::from_millis(10));
                    let _ = draw(display, display.functions_map[&display.display]);
                }
                KeyCode::Char(' ') => {
                    let todo_id = &display.todos[(display.current_todo / 2) as usize - 1]["id"];
                    let _ = database::done(database, todo_id.as_str().parse().unwrap());

                    initialize_all_data(database, display).unwrap();

                    let _ = draw(display, display.functions_map[&display.display]);
                }
                KeyCode::Enter => {
                    display.display = DisplayLocation::Todo;
                    let id = &display.todos[(display.current_todo / 2) as usize - 1]["id"];
                    display.todo_item = database::get_one_todo(database, id.parse().unwrap());
                    let _ = draw(display, display.functions_map[&display.display]);
                }
                KeyCode::Backspace => {
                    if display.display != DisplayLocation::Main {
                        display.display = DisplayLocation::Main;

                        draw(display, components::main_component).unwrap();

                        display.text.title = String::new();
                        display.text.description = String::new();
                    }
                }
                KeyCode::Char('a') => {
                    display.screen.execute(Show).unwrap();
                    display.display = DisplayLocation::Add;

                    draw(display, components::add_todo_component).unwrap();

                    read_text_boxes(display, database).unwrap();
                    display.display = DisplayLocation::Main;
                    display.todos = database::get_data(database, display.maxl);
                    display.current_todo = 2;
                    sleep(Duration::from_millis(10));
                    draw(display, display.functions_map[&display.display]).unwrap();
                    display.screen.execute(Show).unwrap();
                }
                KeyCode::Char('d') => {
                    let todo_id = &display.todos[(display.current_todo / 2) as usize - 1]["id"];
                    let _ = database::delete(database, todo_id.as_str().parse().unwrap());

                    initialize_all_data(database, display).unwrap();
                    let _ = draw(display, display.functions_map[&display.display]);
                }
                _ => {}
            },
            Event::Resize(col, line) => {
                display.maxc = col;
                display.maxl = line;
                draw(display, display.functions_map[&display.display]).unwrap();
            }
            _ => {}
        }
    }
}

pub fn draw(
    display: &mut Display,
    components: fn(display: &mut Display, stdout: &mut io::Stdout, z: &mut u16) -> io::Result<()>,
) -> io::Result<()> {
    let mut stdout = io::stdout();
    display
        .screen
        .execute(terminal::Clear(terminal::ClearType::All))?;
    let mut z: u16 = 0;

    components(display, &mut stdout, &mut z)?;

    let horizontal_border = "═".magenta();
    let vertical_border = "║".magenta();
    let top_left_corner = "╔".magenta();
    let top_right_corner = "╗".magenta();
    let bottom_left_corner = "╚".magenta();
    let bottom_right_corner = "╝".magenta();

    stdout
        .queue(cursor::MoveTo(0, 0))?
        .queue(style::PrintStyledContent(top_left_corner))?;

    stdout
        .queue(cursor::MoveTo(display.maxc - 1, 0))?
        .queue(style::PrintStyledContent(top_right_corner))?;

    stdout
        .queue(cursor::MoveTo(0, display.maxl - 1))?
        .queue(style::PrintStyledContent(bottom_left_corner))?;

    stdout
        .queue(cursor::MoveTo(display.maxc - 1, display.maxl - 1))?
        .queue(style::PrintStyledContent(bottom_right_corner))?;

    for x in 1..display.maxc - 1 {
        stdout
            .queue(cursor::MoveTo(x, 0))?
            .queue(style::PrintStyledContent(horizontal_border))?;
        stdout
            .queue(cursor::MoveTo(x, display.maxl - 1))?
            .queue(style::PrintStyledContent(horizontal_border))?;
    }

    for y in 1..display.maxl - 1 {
        stdout
            .queue(cursor::MoveTo(0, y))?
            .queue(style::PrintStyledContent(vertical_border))?;
        stdout
            .queue(cursor::MoveTo(display.maxc - 1, y))?
            .queue(style::PrintStyledContent(vertical_border))?;
    }

    stdout.flush()?;
    Ok(())
}
