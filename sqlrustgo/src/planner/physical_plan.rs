use crate::parser::ast::{self, Value};

#[derive(Debug, Clone)]
pub enum PhysicalPlan {
    Scan {
        table_name: String,
        columns: Vec<String>,
        filter: Option<ast::Expr>,
    },
    Insert {
        table_name: String,
        values: Vec<Value>,
    },
    Update {
        table_name: String,
        set_clauses: Vec<(String, Value)>,
        filter: Option<ast::Expr>,
    },
    Delete {
        table_name: String,
        filter: Option<ast::Expr>,
    },
    CreateTable {
        table_name: String,
        columns: Vec<ast::ColumnDefinition>,
    },
}
