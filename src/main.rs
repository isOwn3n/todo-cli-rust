use crossterm::{
    cursor::Show,
    terminal::{disable_raw_mode, enable_raw_mode},
    ExecutableCommand,
};
use display::Display;

mod database;
mod display;

fn main() -> std::io::Result<()> {
    clearscreen::clear().expect("failed to clear screen");
    // init the screen
    let (mut sc, maxc, maxl, functions) = display::init_screen()?;
    enable_raw_mode()?;

    // Init DB
    let mut database = database::init_database().unwrap();

    database::create_table(&mut database).unwrap();

    // Get todo data
    let todos = database::get_data(&mut database, maxl);
    let mut todos_count = database::get_count(&mut database).unwrap();

    // Display data
    let mut display = Display::new(
        maxc,
        maxl,
        &mut (todos.len() as u16),
        &mut todos_count,
        todos.clone(),
        functions,
        &mut sc,
    );

    let _ = display::draw::draw(&mut display, display::components::main_component);

    let _ = display.display_holder(&mut database);

    sc.execute(Show)?;
    disable_raw_mode()?;
    clearscreen::clear().expect("");
    Ok(())
}
