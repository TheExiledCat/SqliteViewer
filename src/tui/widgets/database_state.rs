use std::ptr;

use indexmap::IndexMap;
use ratatui::{
    layout::{Constraint, Layout},
    style::{Style, Stylize},
    text::Text,
    widgets::{Block, Cell, List, Paragraph, Row, StatefulWidget, Table, TableState, Widget},
};

use crate::data::{
    sqlite_database::{SqliteDatabaseState, SqliteDatabaseStateMode},
    sqlite_table::SqliteTable,
};

impl SqliteDatabaseState {
    pub fn widget(&self) -> SqliteDatabaseStateWidget {
        let state = self;
        return SqliteDatabaseStateWidget {
            database_state: state,
        };
    }
}

pub struct SqliteDatabaseStateWidget<'a> {
    database_state: &'a SqliteDatabaseState,
}

impl<'a> Widget for SqliteDatabaseStateWidget<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let main_layout =
            Layout::horizontal([Constraint::Percentage(25), Constraint::Fill(1)]).split(area);
        let main_block = Block::bordered();
        //Table list system
        let mut list_block = main_block.clone();
        if let SqliteDatabaseStateMode::TABLE_SELECTION = self.database_state.mode {
            list_block = list_block.red();
        }
        let list = List::new(self.database_state.tables.iter().enumerate().map(|(i, t)| {
            let mut text = Text::raw(t.name.as_str()).centered();

            if let Some(selected) = self.database_state.selected_table {
                if selected == i {
                    text = text.style(Style::new().reversed());

                    match self.database_state.mode {
                        SqliteDatabaseStateMode::QUERY_TOOL => text = text.red(),
                        _ => (),
                    }
                }
            }

            return text;
        }))
        .reset()
        .block(list_block);
        Widget::render(list, main_layout[0], buf);

        //Query system
        let query_layout = Layout::vertical([Constraint::Percentage(30), Constraint::Fill(1)])
            .split(main_layout[1]);
        let mut query_block = main_block.clone();
        if let SqliteDatabaseStateMode::QUERY_TOOL = self.database_state.mode {
            query_block = query_block.red();
        }
        Paragraph::new(format!("{}█", self.database_state.current_query))
            .reset()
            .block(query_block)
            .render(query_layout[0], buf);
        if let Some(err) = &self.database_state.error {
            Paragraph::new(format!("ERROR: {}", err.to_string())).render(query_layout[1], buf);
            return;
        }
        if let Some(queried) = &self.database_state.queried_table_state {
            if let Some(affected) = queried.rows_affected {
                // just show changed rows:
                Paragraph::new(format!("QUERY OK: {} rows affected", affected))
                    .render(query_layout[1], buf);
            } else if queried.rows.len() > 0 {
                // show table
                const max_spacing: u8 = 30;
                let rows: Vec<Row> = queried
                    .rows
                    .iter()
                    .map(|map| {
                        let string_row = map.iter().map(|(k, v)| {
                            let content = match v {
                                rusqlite::types::Value::Null => "NULL".to_string(),
                                rusqlite::types::Value::Integer(i) => i.to_string(),
                                rusqlite::types::Value::Real(f) => f.to_string(),
                                rusqlite::types::Value::Text(s) => s.to_string(),
                                rusqlite::types::Value::Blob(items) => "$$BIN$$".to_string(),
                            };
                            return format!(
                                "{:<width$}|",
                                content,
                                width = (max_spacing - 1) as usize
                            );
                        });
                        return string_row;
                    })
                    .map(|cells| Row::new(cells))
                    .collect();
                let header_row: Vec<&str> = queried
                    .rows
                    .first()
                    .unwrap()
                    .keys()
                    .map(|s| s.as_str())
                    .collect();

                let constraints = [Constraint::Max(max_spacing as u16)].repeat(rows.len());
                Widget::render(
                    Table::new(rows, constraints)
                        .header(Row::new(header_row).style(Style::new().reversed()))
                        .block(main_block),
                    query_layout[1],
                    buf,
                );
            } else {
                Paragraph::new("QUERY OK: 0 Rows returned").render(query_layout[1], buf);
            }
        }
    }
}
