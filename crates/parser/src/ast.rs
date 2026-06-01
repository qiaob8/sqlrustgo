// 抽象语法树

#[derive(Debug, PartialEq)]
pub enum Statement {
    Select {
        table_name: String,
        columns: Vec<String>,
        where_clause: Option<Expr>,
    },
    Insert {
        table_name: String,
        values: Vec<Value>,
    },
    Update {
        table_name: String,
        set_clauses: Vec<(String, Value)>,
        where_clause: Option<Expr>,
    },
    Delete {
        table_name: String,
        where_clause: Option<Expr>,
    },
    CreateTable {
        table_name: String,
        columns: Vec<ColumnDefinition>,
    },
}

#[derive(Debug, PartialEq)]
pub struct ColumnDefinition {
    pub name: String,
    pub data_type: DataType,
}

#[derive(Debug, PartialEq)]
pub enum DataType {
    Int,
    Varchar(usize),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Equal(Box<Expr>, Box<Expr>),
    LessThan(Box<Expr>, Box<Expr>),
    GreaterThan(Box<Expr>, Box<Expr>),
    Column(String),
    Value(Value),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(i32),
    String(String),
}

// ==================== AST测试模块 ====================

#[cfg(test)]
mod ast_tests {
    use super::*;

    // ==================== 1. Statement枚举测试 ====================

    #[test]
    fn test_statement_select_equality() {
        let stmt1 = Statement::Select {
            table_name: "users".to_string(),
            columns: vec!["id".to_string(), "name".to_string()],
            where_clause: None,
        };
        
        let stmt2 = Statement::Select {
            table_name: "users".to_string(),
            columns: vec!["id".to_string(), "name".to_string()],
            where_clause: None,
        };
        
        assert_eq!(stmt1, stmt2);
    }

    #[test]
    fn test_statement_select_inequality() {
        let stmt1 = Statement::Select {
            table_name: "users".to_string(),
            columns: vec!["id".to_string()],
            where_clause: None,
        };
        
        let stmt2 = Statement::Select {
            table_name: "products".to_string(),
            columns: vec!["id".to_string()],
            where_clause: None,
        };
        
        assert_ne!(stmt1, stmt2);
    }

    #[test]
    fn test_statement_insert_equality() {
        let stmt1 = Statement::Insert {
            table_name: "users".to_string(),
            values: vec![Value::Number(1), Value::String("John".to_string())],
        };
        
        let stmt2 = Statement::Insert {
            table_name: "users".to_string(),
            values: vec![Value::Number(1), Value::String("John".to_string())],
        };
        
        assert_eq!(stmt1, stmt2);
    }

    #[test]
    fn test_statement_update_equality() {
        let stmt1 = Statement::Update {
            table_name: "users".to_string(),
            set_clauses: vec![("name".to_string(), Value::String("Jane".to_string()))],
            where_clause: None,
        };
        
        let stmt2 = Statement::Update {
            table_name: "users".to_string(),
            set_clauses: vec![("name".to_string(), Value::String("Jane".to_string()))],
            where_clause: None,
        };
        
        assert_eq!(stmt1, stmt2);
    }

    #[test]
    fn test_statement_delete_equality() {
        let stmt1 = Statement::Delete {
            table_name: "users".to_string(),
            where_clause: None,
        };
        
        let stmt2 = Statement::Delete {
            table_name: "users".to_string(),
            where_clause: None,
        };
        
        assert_eq!(stmt1, stmt2);
    }

    #[test]
    fn test_statement_create_table_equality() {
        let stmt1 = Statement::CreateTable {
            table_name: "users".to_string(),
            columns: vec![
                ColumnDefinition {
                    name: "id".to_string(),
                    data_type: DataType::Int,
                },
                ColumnDefinition {
                    name: "name".to_string(),
                    data_type: DataType::Varchar(255),
                },
            ],
        };
        
        let stmt2 = Statement::CreateTable {
            table_name: "users".to_string(),
            columns: vec![
                ColumnDefinition {
                    name: "id".to_string(),
                    data_type: DataType::Int,
                },
                ColumnDefinition {
                    name: "name".to_string(),
                    data_type: DataType::Varchar(255),
                },
            ],
        };
        
        assert_eq!(stmt1, stmt2);
    }

    // ==================== 2. ColumnDefinition测试 ====================

    #[test]
    fn test_column_definition_int() {
        let col = ColumnDefinition {
            name: "id".to_string(),
            data_type: DataType::Int,
        };
        
        assert_eq!(col.name, "id");
        match col.data_type {
            DataType::Int => {},
            _ => panic!("Expected Int data type"),
        }
    }

    #[test]
    fn test_column_definition_varchar() {
        let col = ColumnDefinition {
            name: "name".to_string(),
            data_type: DataType::Varchar(255),
        };
        
        assert_eq!(col.name, "name");
        match col.data_type {
            DataType::Varchar(length) => assert_eq!(length, 255),
            _ => panic!("Expected Varchar data type"),
        }
    }

    #[test]
    fn test_column_definition_equality() {
        let col1 = ColumnDefinition {
            name: "id".to_string(),
            data_type: DataType::Int,
        };
        
        let col2 = ColumnDefinition {
            name: "id".to_string(),
            data_type: DataType::Int,
        };
        
        assert_eq!(col1, col2);
    }

    // ==================== 3. DataType测试 ====================

    #[test]
    fn test_data_type_int() {
        let dt = DataType::Int;
        match dt {
            DataType::Int => {},
            _ => panic!("Expected Int"),
        }
    }

    #[test]
    fn test_data_type_varchar_different_lengths() {
        let dt1 = DataType::Varchar(100);
        let dt2 = DataType::Varchar(255);
        
        assert_ne!(dt1, dt2);
    }

    #[test]
    fn test_data_type_debug() {
        let dt = DataType::Int;
        let debug_str = format!("{:?}", dt);
        assert_eq!(debug_str, "Int");
    }

    // ==================== 4. Expr测试 ====================

    #[test]
    fn test_expr_equal() {
        let expr = Expr::Equal(
            Box::new(Expr::Column("id".to_string())),
            Box::new(Expr::Value(Value::Number(1))),
        );
        
        match expr {
            Expr::Equal(_, _) => {},
            _ => panic!("Expected Equal expression"),
        }
    }

    #[test]
    fn test_expr_less_than() {
        let expr = Expr::LessThan(
            Box::new(Expr::Column("age".to_string())),
            Box::new(Expr::Value(Value::Number(18))),
        );
        
        match expr {
            Expr::LessThan(_, _) => {},
            _ => panic!("Expected LessThan expression"),
        }
    }

    #[test]
    fn test_expr_greater_than() {
        let expr = Expr::GreaterThan(
            Box::new(Expr::Column("score".to_string())),
            Box::new(Expr::Value(Value::Number(60))),
        );
        
        match expr {
            Expr::GreaterThan(_, _) => {},
            _ => panic!("Expected GreaterThan expression"),
        }
    }

    #[test]
    fn test_expr_column() {
        let expr = Expr::Column("name".to_string());
        
        match expr {
            Expr::Column(name) => assert_eq!(name, "name"),
            _ => panic!("Expected Column expression"),
        }
    }

    #[test]
    fn test_expr_value_number() {
        let expr = Expr::Value(Value::Number(42));
        
        match expr {
            Expr::Value(Value::Number(n)) => assert_eq!(n, 42),
            _ => panic!("Expected Number value"),
        }
    }

    #[test]
    fn test_expr_value_string() {
        let expr = Expr::Value(Value::String("test".to_string()));
        
        match expr {
            Expr::Value(Value::String(s)) => assert_eq!(s, "test"),
            _ => panic!("Expected String value"),
        }
    }

    // ==================== 5. Value测试 ====================

    #[test]
    fn test_value_number_equality() {
        let v1 = Value::Number(42);
        let v2 = Value::Number(42);
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_value_number_inequality() {
        let v1 = Value::Number(42);
        let v2 = Value::Number(100);
        assert_ne!(v1, v2);
    }

    #[test]
    fn test_value_string_equality() {
        let v1 = Value::String("hello".to_string());
        let v2 = Value::String("hello".to_string());
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_value_string_inequality() {
        let v1 = Value::String("hello".to_string());
        let v2 = Value::String("world".to_string());
        assert_ne!(v1, v2);
    }

    #[test]
    fn test_value_number_vs_string() {
        let v1 = Value::Number(42);
        let v2 = Value::String("42".to_string());
        assert_ne!(v1, v2);
    }

    #[test]
    fn test_value_debug_number() {
        let v = Value::Number(42);
        let debug_str = format!("{:?}", v);
        assert_eq!(debug_str, "Number(42)");
    }

    #[test]
    fn test_value_debug_string() {
        let v = Value::String("test".to_string());
        let debug_str = format!("{:?}", v);
        assert_eq!(debug_str, "String(\"test\")");
    }

    // ==================== 6. Clone测试 ====================

    #[test]
    fn test_statement_clone() {
        let stmt1 = Statement::Select {
            table_name: "users".to_string(),
            columns: vec!["id".to_string()],
            where_clause: None,
        };
        
        let stmt2 = stmt1.clone();
        assert_eq!(stmt1, stmt2);
    }

    #[test]
    fn test_value_clone() {
        let v1 = Value::String("test".to_string());
        let v2 = v1.clone();
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_expr_clone() {
        let expr1 = Expr::Column("name".to_string());
        let expr2 = expr1.clone();
        assert_eq!(format!("{:?}", expr1), format!("{:?}", expr2));
    }

    // ==================== 7. 复杂场景测试 ====================

    #[test]
    fn test_complex_select_statement() {
        let stmt = Statement::Select {
            table_name: "users".to_string(),
            columns: vec![
                "id".to_string(),
                "name".to_string(),
                "email".to_string(),
            ],
            where_clause: Some(Expr::Equal(
                Box::new(Expr::Column("id".to_string())),
                Box::new(Expr::Value(Value::Number(1))),
            )),
        };
        
        match stmt {
            Statement::Select { table_name, columns, where_clause } => {
                assert_eq!(table_name, "users");
                assert_eq!(columns.len(), 3);
                assert!(where_clause.is_some());
            }
        }
    }

    #[test]
    fn test_complex_update_statement() {
        let stmt = Statement::Update {
            table_name: "users".to_string(),
            set_clauses: vec![
                ("name".to_string(), Value::String("Alice".to_string())),
                ("age".to_string(), Value::Number(30)),
                ("active".to_string(), Value::Number(1)),
            ],
            where_clause: Some(Expr::Equal(
                Box::new(Expr::Column("id".to_string())),
                Box::new(Expr::Value(Value::Number(1))),
            )),
        };
        
        match stmt {
            Statement::Update { set_clauses, where_clause, .. } => {
                assert_eq!(set_clauses.len(), 3);
                assert!(where_clause.is_some());
            }
        }
    }

    #[test]
    fn test_complex_create_table_statement() {
        let stmt = Statement::CreateTable {
            table_name: "users".to_string(),
            columns: vec![
                ColumnDefinition {
                    name: "id".to_string(),
                    data_type: DataType::Int,
                },
                ColumnDefinition {
                    name: "username".to_string(),
                    data_type: DataType::Varchar(50),
                },
                ColumnDefinition {
                    name: "email".to_string(),
                    data_type: DataType::Varchar(255),
                },
                ColumnDefinition {
                    name: "created_at".to_string(),
                    data_type: DataType::Varchar(50),
                },
            ],
        };
        
        match stmt {
            Statement::CreateTable { columns, .. } => {
                assert_eq!(columns.len(), 4);
            }
        }
    }

    // ==================== 8. 嵌套表达式测试 ====================

    #[test]
    fn test_nested_expressions() {
        let expr = Expr::Equal(
            Box::new(Expr::Column("a".to_string())),
            Box::new(Expr::Value(Value::Number(1))),
        );
        
        let nested = Expr::GreaterThan(
            Box::new(expr),
            Box::new(Expr::Value(Value::Number(0))),
        );
        
        match nested {
            Expr::GreaterThan(left, right) => {
                match *left {
                    Expr::Equal(_, _) => {},
                    _ => panic!("Expected nested Equal"),
                }
                match *right {
                    Expr::Value(Value::Number(n)) => assert_eq!(n, 0),
                    _ => panic!("Expected Number"),
                }
            }
            _ => panic!("Expected GreaterThan"),
        }
    }

    // ==================== 9. 边界条件测试 ====================

    #[test]
    fn test_empty_columns() {
        let stmt = Statement::Select {
            table_name: "test".to_string(),
            columns: vec![],
            where_clause: None,
        };
        
        match stmt {
            Statement::Select { columns, .. } => {
                assert!(columns.is_empty());
            }
        }
    }

    #[test]
    fn test_empty_set_clauses() {
        let stmt = Statement::Update {
            table_name: "test".to_string(),
            set_clauses: vec![],
            where_clause: None,
        };
        
        match stmt {
            Statement::Update { set_clauses, .. } => {
                assert!(set_clauses.is_empty());
            }
        }
    }

    #[test]
    fn test_empty_columns_in_create_table() {
        let stmt = Statement::CreateTable {
            table_name: "test".to_string(),
            columns: vec![],
        };
        
        match stmt {
            Statement::CreateTable { columns, .. } => {
                assert!(columns.is_empty());
            }
        }
    }

    #[test]
    fn test_long_string_value() {
        let long_string = "a".repeat(10000);
        let value = Value::String(long_string.clone());
        
        match value {
            Value::String(s) => assert_eq!(s.len(), 10000),
            _ => panic!("Expected String"),
        }
    }

    #[test]
    fn test_large_number_value() {
        let value = Value::Number(i32::MAX);
        
        match value {
            Value::Number(n) => assert_eq!(n, i32::MAX),
            _ => panic!("Expected Number"),
        }
    }

    #[test]
    fn test_negative_number_value() {
        let value = Value::Number(i32::MIN);
        
        match value {
            Value::Number(n) => assert_eq!(n, i32::MIN),
            _ => panic!("Expected Number"),
        }
    }

    // ==================== 10. Debug格式测试 ====================

    #[test]
    fn test_statement_debug_select() {
        let stmt = Statement::Select {
            table_name: "users".to_string(),
            columns: vec!["id".to_string()],
            where_clause: None,
        };
        
        let debug_str = format!("{:?}", stmt);
        assert!(debug_str.contains("Select"));
        assert!(debug_str.contains("users"));
    }

    #[test]
    fn test_statement_debug_insert() {
        let stmt = Statement::Insert {
            table_name: "users".to_string(),
            values: vec![Value::Number(1)],
        };
        
        let debug_str = format!("{:?}", stmt);
        assert!(debug_str.contains("Insert"));
        assert!(debug_str.contains("users"));
    }

    #[test]
    fn test_statement_debug_update() {
        let stmt = Statement::Update {
            table_name: "users".to_string(),
            set_clauses: vec![("name".to_string(), Value::String("test".to_string()))],
            where_clause: None,
        };
        
        let debug_str = format!("{:?}", stmt);
        assert!(debug_str.contains("Update"));
    }

    #[test]
    fn test_statement_debug_delete() {
        let stmt = Statement::Delete {
            table_name: "users".to_string(),
            where_clause: None,
        };
        
        let debug_str = format!("{:?}", stmt);
        assert!(debug_str.contains("Delete"));
    }

    #[test]
    fn test_statement_debug_create_table() {
        let stmt = Statement::CreateTable {
            table_name: "users".to_string(),
            columns: vec![
                ColumnDefinition {
                    name: "id".to_string(),
                    data_type: DataType::Int,
                },
            ],
        };
        
        let debug_str = format!("{:?}", stmt);
        assert!(debug_str.contains("CreateTable"));
    }

    #[test]
    fn test_expr_debug_equal() {
        let expr = Expr::Equal(
            Box::new(Expr::Column("a".to_string())),
            Box::new(Expr::Value(Value::Number(1))),
        );
        
        let debug_str = format!("{:?}", expr);
        assert!(debug_str.contains("Equal"));
    }

    #[test]
    fn test_expr_debug_less_than() {
        let expr = Expr::LessThan(
            Box::new(Expr::Column("a".to_string())),
            Box::new(Expr::Value(Value::Number(1))),
        );
        
        let debug_str = format!("{:?}", expr);
        assert!(debug_str.contains("LessThan"));
    }

    #[test]
    fn test_expr_debug_greater_than() {
        let expr = Expr::GreaterThan(
            Box::new(Expr::Column("a".to_string())),
            Box::new(Expr::Value(Value::Number(1))),
        );
        
        let debug_str = format!("{:?}", expr);
        assert!(debug_str.contains("GreaterThan"));
    }

    #[test]
    fn test_expr_debug_column() {
        let expr = Expr::Column("test".to_string());
        let debug_str = format!("{:?}", expr);
        assert!(debug_str.contains("Column"));
        assert!(debug_str.contains("test"));
    }

    #[test]
    fn test_expr_debug_value() {
        let expr = Expr::Value(Value::Number(42));
        let debug_str = format!("{:?}", expr);
        assert!(debug_str.contains("Value"));
        assert!(debug_str.contains("42"));
    }

    #[test]
    fn test_column_definition_debug() {
        let col = ColumnDefinition {
            name: "id".to_string(),
            data_type: DataType::Int,
        };
        
        let debug_str = format!("{:?}", col);
        assert!(debug_str.contains("ColumnDefinition"));
        assert!(debug_str.contains("id"));
        assert!(debug_str.contains("Int"));
    }
}
