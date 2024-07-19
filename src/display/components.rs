use std::io::{self, Stdout};

use crossterm::{
    cursor,
    style::{self, Stylize},
    QueueableCommand,
};

use super::Display;

pub fn main_components(
    display: &mut Display,
    stdout: &mut Stdout,
    y: u16,
    z: &mut u16,
) -> io::Result<()> {
    let cursor_start = 2;
    let cursor_row = display.current_todo;
    let id_start = 4;
    let is_done_start = 10;
    let title_start = 18;

    if *z != y / 2 {
        *z = y / 2;
        let todo_index = (*z - 1) as usize;
        if display.todos.len() > todo_index {
            stdout
                .queue(cursor::MoveTo(cursor_start, cursor_row))?
                .queue(style::PrintStyledContent("*".yellow()))?;

            let is_done = &display.todos[todo_index]["is_done"];

            stdout
                .queue(cursor::MoveTo(id_start, y))?
                .queue(style::PrintStyledContent(
                    (display.todos[todo_index]["id"].as_str()).yellow(),
                ))?;
            stdout
                .queue(cursor::MoveTo(is_done_start, y))?
                .queue(style::PrintStyledContent(if (is_done) == "true" {
                    "[x]".white()
                } else {
                    "[ ]".white()
                }))?;

            stdout
                .queue(cursor::MoveTo(title_start, y))?
                .queue(style::PrintStyledContent(
                    display.todos[todo_index]["title"].as_str().cyan(),
                ))?;
        }
    }
    Ok(())
}

// TODO: Complete this part of code
pub fn todo_component(
    display: &mut Display,
    stdout: &mut Stdout,
    _: u16,
    _: &mut u16,
) -> io::Result<()> {
    let todo = &display.todo_item;

    let start_x = 3;
    let start_y = 2;
    let spacing_x = 15; // Horizontal spacing between items in the row
    let spacing_y = 3;

    for (index, (_, value)) in todo.iter().enumerate() {
        let x_position = start_x + (index % 3) as u16 * spacing_x;
        let y_position = start_y + (index / 3) as u16 * spacing_y;

        stdout
            .queue(cursor::MoveTo(x_position, y_position))?
            .queue(style::PrintStyledContent(value.clone().yellow()))?;
    }
    Ok(())
}

pub fn add_todo_component(
    display: &mut Display,
    stdout: &mut Stdout,
    _: u16,
    _: &mut u16,
) -> io::Result<()> {
    let maxc_from: u16 = 17;
    let mut maxc_to: u16 = display.maxc - 3;
    let line_number_from: u16 = 7;
    let mut line_number_to: u16 = display.maxl - 5;

    stdout
        .queue(cursor::MoveTo(3, 2))?
        .queue(style::PrintStyledContent("Title:".bold()))?;

    if display.maxc > 100 {
        maxc_to = display.maxc / 2;
        line_number_to = 15;
    }

    for x in maxc_from..maxc_to {
        stdout
            .queue(cursor::MoveTo(x, 2))?
            .queue(style::PrintStyledContent("_".magenta()))?;
    }

    stdout
        .queue(cursor::MoveTo(3, line_number_from))?
        .queue(style::PrintStyledContent("Description:".bold()))?;

    for x in maxc_from..maxc_to {
        for y in line_number_from..line_number_to {
            stdout
                .queue(cursor::MoveTo(x, y))?
                .queue(style::PrintStyledContent("_".magenta()))?;
        }
    }

    Ok(())
}
