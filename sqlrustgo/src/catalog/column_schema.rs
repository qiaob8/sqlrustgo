use crate::parser::ast::DataType;

pub struct ColumnSchema {
    pub name: String,
    pub data_type: DataType,
    pub nullable: bool,
    pub primary_key: bool,
}

impl ColumnSchema {
    pub fn new(name: &str, data_type: DataType) -> Self {
        Self {
            name: name.to_string(),
            data_type,
            nullable: true,
            primary_key: false,
        }
    }
    
    pub fn set_nullable(&mut self, nullable: bool) {
        self.nullable = nullable;
    }
    
    pub fn set_primary_key(&mut self, primary_key: bool) {
        self.primary_key = primary_key;
    }
}
