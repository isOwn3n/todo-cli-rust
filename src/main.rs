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

    // Init DB
    let mut database = database::init_database().unwrap();

    database::create_table(&mut database).unwrap();

    // Get todo data
    let todos = database::get_data(&mut database, maxl);
    let mut todos_count = database::get_count(&mut database).unwrap();

    // Display data
    let mut display = Display::new(maxc, maxl, &mut todos_count, todos.clone());

    let _ = display::draw::draw(&mut display, display::components::main_components);

    let _ = display.display_holder(&mut sc, &mut database);

    sc.execute(Show)?;
    disable_raw_mode()?;
    clearscreen::clear().expect("");
    Ok(())
}
