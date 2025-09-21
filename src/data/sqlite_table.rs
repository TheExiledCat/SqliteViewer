use std::rc::Rc;

use indexmap::IndexMap;
use rusqlite::{Connection, types::Value};

pub struct SqliteTable {
    pub name: String,
    connection: Rc<Connection>,
}
impl SqliteTable {
    pub fn new(name: String, connection: Rc<Connection>) -> Self {
        return Self { name, connection };
    }

    pub fn columns(&self) -> IndexMap<String, SqliteColumn> {
        let mut map = IndexMap::new();

        let column_select_query = format!("PRAGMA table_info({})", &self.name);

        let mut stmt = self.connection.prepare(&column_select_query).unwrap();
        let column_count = stmt.column_count();
        let mut rows = stmt.query([]).unwrap();
        while let Ok(Some(row)) = rows.next() {
            let name: String = row.get(1).unwrap();
            let declared_type = row.get(2).unwrap();
            let not_null = row.get(3).unwrap();
            let default_value = row.get(4).unwrap();
            let pk = row.get(5).unwrap();
            map.insert(
                name.clone(),
                SqliteColumn::new(name, declared_type, not_null, default_value, pk),
            );
        }
        return map;
    }
}

#[derive(Debug)]
pub struct SqliteColumn {
    pub name: String,
    pub declared_type: String,
    pub not_null: bool,
    pub default_value: Value,
    pub pk: usize,
}
impl SqliteColumn {
    pub fn new(
        name: String,
        declared_type: String,
        not_null: bool,
        default_value: Value,
        pk: usize,
    ) -> Self {
        return Self {
            name,
            declared_type,
            not_null,
            default_value,
            pk,
        };
    }
}
