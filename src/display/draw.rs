use crossterm::{
    cursor::{self},
    event::{poll, read, Event, KeyCode},
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};

use std::{
    io::{self, Write},
    time::Duration,
};

use super::{Display, DisplayLocation};

pub fn handle_pressed_keys(display: &mut Display) {
    if poll(Duration::from_millis(10)).unwrap() {
        let key = read().unwrap();

        while poll(Duration::from_millis(0)).unwrap() {
            let _ = read();
        }

        match key {
            Event::Key(event) => match event.code {
                KeyCode::Down => {
                    if (display.todos_total_len * 2) - 1 < display.current_todo {
                        display.current_todo = 2;
                    } else {
                        display.current_todo += 2;
                    }
                    let _ = draw(display);
                }
                KeyCode::Up => {
                    if (display.todos_total_len * 2) - 1 == display.current_todo {
                        display.current_todo -= 2;
                    } else if 2 >= display.current_todo {
                        display.current_todo = display.todos_total_len * 2;
                    } else {
                        display.current_todo -= 2;
                    }
                    let _ = draw(display);
                }
                KeyCode::Char('q') => display.display = DisplayLocation::Quit,
                KeyCode::Char('r') => {}
                _ => {}
            },
            _ => {}
        }
    }
}

pub fn draw(display: &mut Display) -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    let mut z = 0;
    let cursor_start = 2;
    let cursor_row = display.current_todo;
    let id_start = 4;
    let is_done_start = 10;
    let title_done_start = 18;
    for y in 0..display.maxl {
        for x in 0..display.maxc {
            if (y == 0 || y == display.maxl - 1) || (x == 0 || x == display.maxc - 1) {
                if z != y / 2 {
                    z = y / 2;
                    let todo_index = (z - 1) as usize;
                    if display.todos.len() > todo_index {
                        stdout
                            .queue(cursor::MoveTo(cursor_start, cursor_row))?
                            .queue(style::PrintStyledContent("*".yellow()))?;

                        let is_done = &display.todos[todo_index]["is_done"];

                        stdout.queue(cursor::MoveTo(id_start, y))?.queue(
                            style::PrintStyledContent(
                                (display.todos[todo_index]["id"].as_str()).yellow(),
                            ),
                        )?;
                        stdout.queue(cursor::MoveTo(is_done_start, y))?.queue(
                            style::PrintStyledContent(if (is_done) == "true" {
                                "[x]".white()
                            } else {
                                "[ ]".white()
                            }),
                        )?;

                        stdout.queue(cursor::MoveTo(title_done_start, y))?.queue(
                            style::PrintStyledContent(
                                display.todos[todo_index]["title"].as_str().cyan(),
                            ),
                        )?;
                    }
                }
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
