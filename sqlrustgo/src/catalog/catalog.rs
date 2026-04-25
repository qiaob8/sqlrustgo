use std::collections::HashMap;
use super::{TableSchema, ColumnSchema};

pub struct Catalog {
    tables: HashMap<String, TableSchema>,
}

impl Catalog {
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
        }
    }
    
    pub fn get_table(&self, table_name: &str) -> Result<Option<&TableSchema>, String> {
        Ok(self.tables.get(table_name))
    }
    
    pub fn create_table(&mut self, table_name: &str, columns: Vec<ColumnSchema>) -> Result<(), String> {
        let schema = TableSchema {
            name: table_name.to_string(),
            columns,
        };
        self.tables.insert(table_name.to_string(), schema);
        Ok(())
    }
    
    pub fn drop_table(&mut self, table_name: &str) -> Result<(), String> {
        self.tables.remove(table_name);
        Ok(())
    }
    
    pub fn list_tables(&self) -> Vec<String> {
        self.tables.keys().cloned().collect()
    }
}
