use super::ColumnSchema;

pub struct TableSchema {
    pub name: String,
    pub columns: Vec<ColumnSchema>,
}
