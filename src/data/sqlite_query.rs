use std::ops::Deref;

use indexmap::IndexMap;
use rusqlite::{Row, Rows, types::Value};

pub struct SqliteQueryResult {
    pub rows: Vec<IndexMap<String, Value>>,
    pub rows_affected: Option<usize>,
}
impl SqliteQueryResult {
    pub fn new(mut rows: Rows, column_names: Vec<String>) -> Self {
        let mut result = Vec::new();
        let column_count = column_names.len();
        while let Ok(Some(row)) = rows.next() {
            let mut map = IndexMap::new();
            for col in 0..column_count {
                map.insert(column_names[col].clone(), row.get(col).unwrap());
            }
            result.push(map);
        }

        return SqliteQueryResult {
            rows: result,
            rows_affected: None,
        };
    }
    pub fn mutated(rows_affected: usize) -> Self {
        return Self {
            rows: Vec::new(),
            rows_affected: Some(rows_affected),
        };
    }
    pub fn is_readonly(&self) -> bool {
        return self.rows_affected.is_none();
    }
    pub fn window(&self, offset: usize, max_length: u8) -> &[IndexMap<String, Value>] {
        let max_window_size = self.rows.len() - offset;
        if (max_length as usize) < max_window_size {
            return &self.rows[offset..offset + max_length as usize];
        }

        return &self.rows[offset..offset + max_window_size as usize];
    }
}
