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

#[derive(Debug, PartialEq)]
pub enum Value {
    Number(i32),
    String(String),
}
