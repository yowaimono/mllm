use rusqlite::{Connection, Result};
use std::collections::HashMap;

pub struct SqliteTool {
    conn: Connection,
}

impl SqliteTool {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(SqliteTool { conn })
    }

    pub fn get_tables(&self) -> Result<HashMap<String, Vec<String>>> {
        let mut tables = HashMap::new();
        
        let mut stmt = self.conn.prepare(
            "SELECT name, sql FROM sqlite_master WHERE type='table'"
        )?;
        
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;

        for row in rows {
            let (table_name, create_sql) = row?;
            let columns = self.get_table_columns(&table_name)?;
            tables.insert(table_name, columns);
        }

        Ok(tables)
    }

    fn get_table_columns(&self, table_name: &str) -> Result<Vec<String>> {
        let mut columns = Vec::new();
        let mut stmt = self.conn.prepare(&format!("PRAGMA table_info({})", table_name))?;
        
        let rows = stmt.query_map([], |row| {
            Ok(row.get::<_, String>(1)?) // column name is at index 1
        })?;

        for row in rows {
            columns.push(row?);
        }

        Ok(columns)
    }
}
