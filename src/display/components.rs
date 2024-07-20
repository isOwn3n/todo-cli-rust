use std::io::{self, Stdout, Write};

use crossterm::{
    cursor,
    style::{self, Stylize},
    ExecutableCommand, QueueableCommand,
};

use super::Display;

// TODO: Add scroll to main page
pub fn main_component(display: &mut Display, stdout: &mut Stdout, z: &mut u16) -> io::Result<()> {
    let cursor_start = 2;
    let cursor_row = display.current_todo;
    let id_start = 4;
    let is_done_start = 10;
    let title_start = 18;

    let num_todos = display.todos_len.into();

    let len = if display.maxl % 2 != 0 {
        display.maxl - 1
    } else {
        display.maxl
    };

    clearscreen::clear().expect("");
    for y in (0..len).step_by(2) {
        let current_z = y / 2;
        if *z != current_z {
            *z = current_z;
            let todo_index = (*z - 1) as usize;

            if todo_index < num_todos {
                let todo = &display.todos[todo_index];

                stdout
                    .queue(cursor::MoveTo(cursor_start, cursor_row))?
                    .queue(style::PrintStyledContent("*".yellow()))?
                    .queue(cursor::MoveTo(id_start, y))?
                    .queue(style::PrintStyledContent(todo["id"].as_str().yellow()))?
                    .queue(cursor::MoveTo(is_done_start, y))?
                    .queue(style::PrintStyledContent(
                        if todo["is_done"].as_str() == "true" {
                            "[x]".green()
                        } else {
                            "[ ]".red()
                        },
                    ))?
                    .queue(cursor::MoveTo(title_start, y))?
                    .queue(style::PrintStyledContent(todo["title"].as_str().blue()))?;
            }
        }
    }

    Ok(())
}

// TODO: Complete this part of code
pub fn todo_component(display: &mut Display, stdout: &mut Stdout, _: &mut u16) -> io::Result<()> {
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
    _: &mut u16,
) -> io::Result<()> {
    display.text.init();
    let title_len = display.text.title.len();
    let desc_len = display.text.description.len();
    let maxc_from: u16 = 17;
    let mut maxc_to: u16 = display.maxc - 3;

    let line_number_from: u16 = 7;

    if display.maxc > 100 && display.maxl > 25 {
        maxc_to = display.maxc / 2;
    }
    display.text.input_col_length = maxc_to - maxc_from;
    display.screen.execute(cursor::MoveTo(maxc_from, 2))?;
    display.screen.flush()?;
    display.screen.execute(cursor::Show)?;

    let title_maxc_from: u16 = maxc_from + title_len as u16;
    let desc_maxc_from: u16 = maxc_from + desc_len as u16;

    // Title
    stdout
        .queue(cursor::MoveTo(3, 2))?
        .queue(style::PrintStyledContent("Title:".bold()))?;

    stdout
        .queue(cursor::MoveTo(maxc_from, 2))?
        .queue(style::PrintStyledContent(
            display.text.display_title.clone().magenta(),
        ))?;
    // End Title

    // Description
    stdout
        .queue(cursor::MoveTo(3, line_number_from))?
        .queue(style::PrintStyledContent("Description:".bold()))?;

    stdout
        .queue(cursor::MoveTo(maxc_from, line_number_from))?
        .queue(style::PrintStyledContent(
            display.text.display_description.clone().magenta(),
        ))?;
    // End Description

    let line = "_".magenta();

    // Title Lines Creator
    for x in title_maxc_from..maxc_to {
        stdout
            .queue(cursor::MoveTo(x, 2))?
            .queue(style::PrintStyledContent(line))?;
    }

    // Description Lines Creator
    for x in desc_maxc_from..maxc_to {
        stdout
            .queue(cursor::MoveTo(x, line_number_from))?
            .queue(style::PrintStyledContent(line))?
            .flush()?;
    }

    Ok(())
}
