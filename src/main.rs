use std::io;

use clap::Parser;
use cli::CommandLine;
use data::sqlite_database::SqliteDatabase;
use rusqlite::Connection;
use tui::app::App;

mod cli;
mod data;
mod tui;
fn main() -> Result<(), rusqlite::Error> {
    let command_line = CommandLine::parse();
    let conn = Connection::open(command_line.file_path)?;

    let db = SqliteDatabase::new(conn);
    let terminal = ratatui::init();
    let mut app = App::new(terminal, 120, 10, db);

    while app.draw() {}
    ratatui::restore();
    println!("Application exit requested");
    return Ok(());
}
