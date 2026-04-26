use crate::parser::ast::{self, Value};

#[derive(Debug, Clone)]
pub enum PhysicalPlan {
    Scan {
        table_name: String,
        columns: Vec<String>,
        filter: Option<ast::Expr>,
    },
    Project {
        input: Box<PhysicalPlan>,
        columns: Vec<String>,
    },
    Filter {
        input: Box<PhysicalPlan>,
        condition: ast::Expr,
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

impl PhysicalPlan {
    pub fn cost(&self) -> f64 {
        // 简单的成本估算
        match self {
            PhysicalPlan::Scan { table_name: _, columns: _, filter: _ } => {
                // 扫描操作的成本
                10.0
            }
            PhysicalPlan::Project { input, columns: _ } => {
                // 投影操作的成本
                input.cost() + 1.0
            }
            PhysicalPlan::Filter { input, condition: _ } => {
                // 过滤操作的成本
                input.cost() + 2.0
            }
            _ => 1.0,
        }
    }
}
