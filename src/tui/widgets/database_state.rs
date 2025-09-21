use indexmap::IndexMap;
use ratatui::{
    layout::{Constraint, Layout},
    style::{Style, Stylize},
    widgets::{Block, Cell, Paragraph, Row, StatefulWidget, Table, TableState, Widget},
};

use crate::data::sqlite_database::SqliteDatabaseState;

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
        let layout =
            Layout::vertical([Constraint::Percentage(30), Constraint::Fill(1)]).split(area);
        Paragraph::new(format!("{}", self.database_state.current_query))
            .block(Block::bordered())
            .render(layout[0], buf);
        if let Some(err) = &self.database_state.error {
            Paragraph::new(format!("ERROR: {}", err.to_string())).render(layout[1], buf);
            return;
        }
        if let Some(queried) = &self.database_state.queried_table_state {
            if let Some(affected) = queried.rows_affected {
                // just show changed rows:
                Paragraph::new(format!("QUERY OK: {} rows affected", affected))
                    .render(layout[1], buf);
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
                        .header(Row::new(header_row).style(Style::new().reversed())),
                    layout[1],
                    buf,
                );
            } else {
                Paragraph::new("QUERY OK: 0 Rows returned").render(layout[1], buf);
            }
        }
    }
}
