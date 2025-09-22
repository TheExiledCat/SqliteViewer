use ratatui::{
    layout::{Constraint, Layout},
    style::{Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, List, Padding, Paragraph, Row, Table, Widget},
};
use sqlparser::{
    dialect::SQLiteDialect,
    keywords::Keyword,
    tokenizer::{Token, Tokenizer},
};

use crate::data::sqlite_database::{SqliteDatabaseState, SqliteDatabaseStateMode};

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
        let mut query_block = main_block.clone().padding(Padding::horizontal(1));
        if let SqliteDatabaseStateMode::QUERY_TOOL = self.database_state.mode {
            query_block = query_block.red();
        }

        let query_tokens = match Tokenizer::new(
            &SQLiteDialect {},
            &self.database_state.current_query,
        )
        .tokenize()
        {
            Ok(tokens) => tokens,
            Err(_) => {
                // Fallback: just return everything as raw text
                vec![Token::Word(sqlparser::tokenizer::Word {
                    value: self.database_state.current_query.to_string(),
                    quote_style: None,
                    keyword: Keyword::NoKeyword,
                })]
            }
        };

        let mut column = 0;
        let query_text: Vec<Span> = query_tokens
            .iter()
            .enumerate()
            .flat_map(|(i, t)| {
                let len = t.to_string().chars().count();
                let mut span = [match t {
                    Token::Word(w) => {
                        if let Keyword::NoKeyword = w.keyword {
                            if let Some(_) = self.database_state.tables.iter().find(|table| {
                                table.name.to_lowercase() == t.to_string().to_lowercase()
                            }) {
                                Span::raw(t.to_string()).red()
                            } else {
                                Span::raw(&w.value)
                            }
                        } else {
                            Span::raw(&w.value).blue()
                        }
                    }
                    _ => Span::from(t.to_string()),
                }]
                .to_vec();
                let (col, row) = self.database_state.current_query_cursor;
                let inner_span = span[0].clone();
                if col >= column && col < column + len {
                    // the cursor should be in this token
                    let style = inner_span.style;
                    let content = inner_span.content.clone();
                    let (left, right) = content.split_at(col - column);
                    let (mid, right) = right.split_at(1);
                    let left_styled = Span::raw(left.to_owned()).style(style.clone());
                    let right_styled = Span::raw(right.to_owned()).style(style.clone());
                    let mid_styled = Span::raw(mid.to_owned()).reset().reversed();
                    span = [left_styled, mid_styled, right_styled].to_vec();
                }
                column += len;
                return span;
            })
            .chain(
                if self.database_state.current_query_cursor.0
                    == self.database_state.current_query.chars().count()
                {
                    let mut extra = Vec::new();
                    extra.push(Span::raw("█").reset());
                    extra
                } else {
                    [].to_vec()
                },
            )
            .collect();

        Paragraph::new(Line::from(query_text))
            .reset()
            .block(query_block)
            .render(query_layout[0], buf);
        if let Some(err) = &self.database_state.error {
            Line::from(vec![
                Span::raw("ERROR").red(),
                Span::raw(": "),
                Span::raw(err.to_string()),
            ])
            .render(query_layout[1], buf);
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
                        .block(main_block.padding(Padding::uniform(1))),
                    query_layout[1],
                    buf,
                );
            } else {
                Line::from(
                    [
                        Span::raw("QUERY "),
                        Span::raw("OK").green(),
                        Span::raw(": 0 Rows returned"),
                    ]
                    .to_vec(),
                )
                .render(query_layout[1], buf);
            }
        }
    }
}

fn get_last_non_whitespace_token(tokens: &[Token], column: usize) -> Option<(usize, &Token)> {
    if tokens.len() == 0 {
        return None;
    }
    let mut i = 0;
    let mut last_non_whitespace = None;
    let mut current_column = 0;
    while current_column <= column {
        let token = &tokens[i];

        if let Token::Whitespace(_) = token {
            i += 1;
            current_column += token.to_string().chars().count();
            continue;
        }
        i += 1;
        current_column += token.to_string().chars().count();
        last_non_whitespace = Some((i, token));
    }
    return last_non_whitespace;
}
