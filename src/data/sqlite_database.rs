use std::{ops::Deref, path::PathBuf, rc::Rc};

use ratatui::crossterm::event::{KeyCode, KeyEvent};
use rusqlite::{Connection, Error};

use super::{sqlite_query::SqliteQueryResult, sqlite_table::SqliteTable};
#[derive(Clone)]
pub struct SqliteDatabase {
    connection: Rc<Connection>,
    pub database_path: PathBuf,
}
impl SqliteDatabase {
    pub fn new(connection: Connection, database_path: PathBuf) -> Self {
        return Self {
            connection: Rc::new(connection),
            database_path,
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

pub enum SqliteDatabaseStateMode {
    TABLE_SELECTION,
    TABLE_OPTION_SELECTION,
    QUERY_TOOL,
}
#[repr(usize)]
pub enum TableOption {
    CREATE = 0,
    CUSTOM = 1,
}
pub struct SqliteDatabaseState {
    connection: Rc<Connection>,
    pub tables: Vec<SqliteTable>,
    pub queried_table_state: Option<SqliteQueryResult>,
    pub current_query: String,
    pub selected_table: Option<usize>,
    pub selected_table_option: Option<TableOption>,
    pub error: Option<Error>,
    pub mode: SqliteDatabaseStateMode,
}
impl SqliteDatabaseState {
    pub fn new(database: &SqliteDatabase) -> Self {
        return Self {
            connection: database.connection.clone(),
            tables: database.tables(),
            queried_table_state: None,
            current_query: String::new(),
            selected_table: None,
            selected_table_option: None,
            mode: SqliteDatabaseStateMode::TABLE_SELECTION,
            error: None,
        };
    }

    pub fn read_keys(&mut self, event: &KeyEvent) {
        match self.mode {
            SqliteDatabaseStateMode::TABLE_SELECTION => match event.code {
                KeyCode::Up => {
                    if let None = self.selected_table {
                        self.selected_table = Some(0)
                    } else if let Some(current_selected) = self.selected_table {
                        self.selected_table = Some(wrap_in_range(
                            current_selected.wrapping_sub(1),
                            0,
                            self.tables.len() - 1,
                        ));
                    }
                }
                KeyCode::Down => {
                    if let None = self.selected_table {
                        self.selected_table = Some(0)
                    } else if let Some(current_selected) = self.selected_table {
                        self.selected_table = Some(wrap_in_range(
                            current_selected.wrapping_add(1),
                            0,
                            self.tables.len() - 1,
                        ));
                    }
                }
                KeyCode::Enter => {
                    if let Some(table_index) = self.selected_table {
                        self.select_table();
                    }
                }

                _ => (),
            },
            SqliteDatabaseStateMode::QUERY_TOOL => {
                match event.code {
                    KeyCode::Backspace => {
                        self.current_query.pop();
                    }
                    KeyCode::Enter => {
                        //execute
                        self.execute();
                    }
                    KeyCode::Char(c) => {
                        self.current_query.push(c);
                    }
                    KeyCode::Esc => self.mode = SqliteDatabaseStateMode::TABLE_SELECTION,
                    _ => (),
                }
            }
            SqliteDatabaseStateMode::TABLE_OPTION_SELECTION => todo!(),
        }
    }

    pub fn execute(&mut self) {
        self.error = None;
        let stmt = self.connection.prepare(&self.current_query);
        if let Err(e) = stmt {
            self.error = Some(e);
            return;
        }
        let mut stmt = stmt.unwrap();

        if stmt.readonly() {
            let column_names = stmt
                .column_names()
                .iter()
                .map(|s| s.deref().to_owned())
                .collect();
            let rows = stmt.query([]);
            if let Ok(rows) = rows {
                self.queried_table_state = Some(SqliteQueryResult::new(rows, column_names));
            } else if let Err(e) = rows {
                self.error = Some(e);
            }
        } else {
            let rows_changed = stmt.execute([]);
            if let Ok(rows_changed) = rows_changed {
                self.queried_table_state = Some(SqliteQueryResult::mutated(rows_changed));
            } else if let Err(e) = rows_changed {
                self.error = Some(e);
            }
        }
    }
    pub fn select_table(&mut self) {
        if self.tables.len() == 0 {
            return;
        }
        self.mode = SqliteDatabaseStateMode::QUERY_TOOL;

        self.current_query = format!(
            "SELECT * FROM {}",
            self.tables[self.selected_table.unwrap_or(0)].name
        );
        self.execute();
    }
    pub fn sync(&mut self, database: &SqliteDatabase) {
        self.tables = database.tables();
    }
}
fn wrap_in_range(x: usize, min: usize, max: usize) -> usize {
    let range = max - min + 1;
    (((x - min) % range + range) % range) + min
}
