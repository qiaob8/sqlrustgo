use crate::parser::ast::{self, Statement, Value};

#[derive(Debug, Clone)]
pub enum LogicalPlan {
    Scan {
        table_name: String,
        columns: Vec<String>,
        filter: Option<ast::Expr>,
    },
    Project {
        input: Box<LogicalPlan>,
        columns: Vec<String>,
    },
    Filter {
        input: Box<LogicalPlan>,
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

impl LogicalPlan {
    pub fn from_statement(statement: &Statement) -> Self {
        match statement {
            Statement::Select { table_name, columns, where_clause } => {
                if let Some(filter_expr) = where_clause {
                    // 创建Filter节点
                    LogicalPlan::Filter {
                        input: Box::new(LogicalPlan::Scan {
                            table_name: table_name.clone(),
                            columns: columns.clone(),
                            filter: None,
                        }),
                        condition: filter_expr.clone(),
                    }
                } else {
                    // 直接返回Scan节点
                    LogicalPlan::Scan {
                        table_name: table_name.clone(),
                        columns: columns.clone(),
                        filter: None,
                    }
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
    
    pub fn optimize(&self) -> LogicalPlan {
        // 实现逻辑优化
        match self {
            LogicalPlan::Scan { table_name, columns, filter } => {
                if let Some(filter_expr) = filter {
                    // 将Scan中的filter转换为Filter节点
                    LogicalPlan::Filter {
                        input: Box::new(LogicalPlan::Scan {
                            table_name: table_name.clone(),
                            columns: columns.clone(),
                            filter: None,
                        }),
                        condition: filter_expr.clone(),
                    }
                } else {
                    self.clone()
                }
            }
            LogicalPlan::Filter { input, condition } => {
                // 优化Filter节点的输入
                let optimized_input = input.optimize();
                LogicalPlan::Filter {
                    input: Box::new(optimized_input),
                    condition: condition.clone(),
                }
            }
            LogicalPlan::Project { input, columns } => {
                // 优化Project节点的输入
                let optimized_input = input.optimize();
                LogicalPlan::Project {
                    input: Box::new(optimized_input),
                    columns: columns.clone(),
                }
            }
            _ => self.clone(),
        }
    }
}
