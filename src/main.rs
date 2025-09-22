use std::{io, process::CommandEnvs};

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
    let file_path = command_line.file_path;
    let conn = Connection::open(&file_path)?;

    let db = SqliteDatabase::new(conn, file_path);
    let terminal = ratatui::init();
    let mut app = App::new(terminal, 120, 3, db);

    while app.draw() {}
    ratatui::restore();
    println!("Application exit requested");
    return Ok(());
}
