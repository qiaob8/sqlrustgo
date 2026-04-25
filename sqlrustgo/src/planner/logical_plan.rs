use crate::parser::ast::{self, Statement, Value};

#[derive(Debug)]
pub enum LogicalPlan {
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

impl LogicalPlan {
    pub fn from_statement(statement: &Statement) -> Self {
        match statement {
            Statement::Select { table_name, columns, where_clause } => {
                LogicalPlan::Scan {
                    table_name: table_name.clone(),
                    columns: columns.clone(),
                    filter: where_clause.clone(),
                }
            }
            Statement::Insert { table_name, values } => {
                LogicalPlan::Insert {
                    table_name: table_name.clone(),
                    values: values.clone(),
                }
            }
            Statement::Update { table_name, set_clauses, where_clause } => {
                LogicalPlan::Update {
                    table_name: table_name.clone(),
                    set_clauses: set_clauses.clone(),
                    filter: where_clause.clone(),
                }
            }
            Statement::Delete { table_name, where_clause } => {
                LogicalPlan::Delete {
                    table_name: table_name.clone(),
                    filter: where_clause.clone(),
                }
            }
            Statement::CreateTable { table_name, columns } => {
                LogicalPlan::CreateTable {
                    table_name: table_name.clone(),
                    columns: columns.clone(),
                }
            }
        }
    }
}
