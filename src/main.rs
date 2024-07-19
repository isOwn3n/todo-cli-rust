use std::io::stdout;

use crossterm::{
    cursor::{Hide, Show},
    terminal::{disable_raw_mode, enable_raw_mode, size},
    ExecutableCommand,
};
use display::Display;

mod args;
mod database;
mod display;

fn main() -> std::io::Result<()> {
    clearscreen::clear().expect("failed to clear screen");
    // init the screen
    let mut sc = stdout();
    let (maxc, maxl) = size().unwrap();
    sc.execute(Hide)?;
    enable_raw_mode()?;

    // Get todo data
    let todos = database::get_data(maxl);
    let mut todos_count = database::get_count().unwrap();

    // Display data
    let mut display = Display::new(
        maxc,
        maxl,
        &mut (todos.len() as u16),
        &mut todos_count,
        todos.clone(),
    );

    let _ = display::draw::draw(&mut display);

    let _ = display.display_holder(&mut sc);

    sc.execute(Show)?;
    disable_raw_mode()?;
    Ok(())
}
