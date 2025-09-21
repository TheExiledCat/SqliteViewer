use std::{ops::Deref, rc::Rc};

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    widgets::Table,
};
use rusqlite::{Connection, types::Value};

use super::{sqlite_query::SqliteQueryResult, sqlite_table::SqliteTable};
pub struct SqliteDatabase {
    connection: Rc<Connection>,
}
impl SqliteDatabase {
    pub fn new(connection: Connection) -> Self {
        return Self {
            connection: Rc::new(connection),
        };
    }
    pub fn tables(&self) -> Vec<SqliteTable> {
        let mut tables = Vec::new();

        // Query all tables
        let mut stmt = self
            .connection
            .prepare(
                "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%';",
            )
            .unwrap();

        let rows: Vec<String> = stmt
            .query_map([], |r| r.get::<usize, String>(0))
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        for row in rows {
            tables.push(SqliteTable::new(row.clone(), self.connection.clone()));
        }
        return tables;
    }
}

pub struct SqliteDatabaseState {
    connection: Rc<Connection>,
    pub tables: Vec<SqliteTable>,
    pub queried_table_state: Option<SqliteQueryResult>,
    pub current_query: String,
}
impl SqliteDatabaseState {
    pub fn new(database: &SqliteDatabase) -> Self {
        return Self {
            connection: database.connection.clone(),
            tables: database.tables(),
            queried_table_state: None,
            current_query: String::new(),
        };
    }

    pub fn read_keys(&mut self, event: &KeyEvent) {
        match event.code {
            KeyCode::Backspace => {
                self.current_query.pop();
            }
            KeyCode::Enter => {
                //execute
            }
            KeyCode::Char(c) => {
                self.current_query.push(c);
            }
            _ => (),
        }
    }

    pub fn execute(&mut self) -> Result<(), rusqlite::Error> {
        let mut stmt = self.connection.prepare(&self.current_query).unwrap();

        if stmt.readonly() {
            let column_names = stmt
                .column_names()
                .iter()
                .map(|s| s.deref().to_owned())
                .collect();
            let rows = stmt.query([])?;
            self.queried_table_state = Some(SqliteQueryResult::new(rows, column_names));
        } else {
            let rows_changed = stmt.execute([])?;
            self.queried_table_state = Some(SqliteQueryResult::mutated(rows_changed));
        }

        return Ok(());
    }
    pub fn select_table(table: &SqliteTable) {
        todo!();
    }
}
