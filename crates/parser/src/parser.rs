// 语法分析器

use super::lexer::{Lexer, Token};
use super::ast::{Statement, ColumnDefinition, DataType, Expr, Value};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            lexer: Lexer::new(""),
            current_token: Token::EOF,
        }
    }
    
    pub fn parse(&mut self, sql: &str) -> Result<Statement, String> {
        self.lexer = Lexer::new(sql);
        self.current_token = self.lexer.next_token();
        
        self.parse_statement()
    }
    
    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.current_token {
            Token::SELECT => self.parse_select(),
            Token::INSERT => self.parse_insert(),
            Token::UPDATE => self.parse_update(),
            Token::DELETE => self.parse_delete(),
            Token::CREATE => self.parse_create_table(),
            _ => Err(format!("Unexpected token: {:?}", self.current_token)),
        }
    }
    
    fn parse_select(&mut self) -> Result<Statement, String> {
        self.eat(Token::SELECT)?;
        
        let columns = self.parse_columns()?;
        
        self.eat(Token::FROM)?;
        
        let table_name = match self.current_token {
            Token::IDENTIFIER(name) => {
                self.advance();
                name
            }
            _ => return Err("Expected table name".to_string()),
        };
        
        let where_clause = if self.current_token == Token::WHERE {
            self.advance();
            Some(self.parse_expr()?)
        } else {
            None
        };
        
        Ok(Statement::Select {
            table_name,
            columns,
            where_clause,
        })
    }
    
    fn parse_insert(&mut self) -> Result<Statement, String> {
        self.eat(Token::INSERT)?;
        self.eat(Token::INTO)?;
        
        let table_name = match self.current_token {
            Token::IDENTIFIER(name) => {
                self.advance();
                name
            }
            _ => return Err("Expected table name".to_string()),
        };
        
        self.eat(Token::VALUES)?;
        self.eat(Token::LEFT_PAREN)?;
        
        let values = self.parse_values()?;
        
        self.eat(Token::RIGHT_PAREN)?;
        
        Ok(Statement::Insert {
            table_name,
            values,
        })
    }
    
    fn parse_update(&mut self) -> Result<Statement, String> {
        self.eat(Token::UPDATE)?;
        
        let table_name = match self.current_token {
            Token::IDENTIFIER(name) => {
                self.advance();
                name
            }
            _ => return Err("Expected table name".to_string()),
        };
        
        self.eat(Token::SET)?;
        
        let set_clauses = self.parse_set_clauses()?;
        
        let where_clause = if self.current_token == Token::WHERE {
            self.advance();
            Some(self.parse_expr()?)
        } else {
            None
        };
        
        Ok(Statement::Update {
            table_name,
            set_clauses,
            where_clause,
        })
    }
    
    fn parse_delete(&mut self) -> Result<Statement, String> {
        self.eat(Token::DELETE)?;
        self.eat(Token::FROM)?;
        
        let table_name = match self.current_token {
            Token::IDENTIFIER(name) => {
                self.advance();
                name
            }
            _ => return Err("Expected table name".to_string()),
        };
        
        let where_clause = if self.current_token == Token::WHERE {
            self.advance();
            Some(self.parse_expr()?)
        } else {
            None
        };
        
        Ok(Statement::Delete {
            table_name,
            where_clause,
        })
    }
    
    fn parse_create_table(&mut self) -> Result<Statement, String> {
        self.eat(Token::CREATE)?;
        self.eat(Token::TABLE)?;
        
        let table_name = match self.current_token {
            Token::IDENTIFIER(name) => {
                self.advance();
                name
            }
            _ => return Err("Expected table name".to_string()),
        };
        
        self.eat(Token::LEFT_PAREN)?;
        
        let columns = self.parse_column_definitions()?;
        
        self.eat(Token::RIGHT_PAREN)?;
        
        Ok(Statement::CreateTable {
            table_name,
            columns,
        })
    }
    
    // 辅助方法
    fn parse_columns(&mut self) -> Result<Vec<String>, String> {
        let mut columns = Vec::new();
        
        if self.current_token == Token::IDENTIFIER("*".to_string()) {
            columns.push("*".to_string());
            self.advance();
        } else {
            loop {
                match self.current_token {
                    Token::IDENTIFIER(name) => {
                        columns.push(name);
                        self.advance();
                    }
                    _ => return Err("Expected column name".to_string()),
                }
                
                if self.current_token != Token::COMMA {
                    break;
                }
                self.advance();
            }
        }
        
        Ok(columns)
    }
    
    fn parse_values(&mut self) -> Result<Vec<Value>, String> {
        let mut values = Vec::new();
        
        loop {
            values.push(self.parse_value()?);
            
            if self.current_token != Token::COMMA {
                break;
            }
            self.advance();
        }
        
        Ok(values)
    }
    
    fn parse_value(&mut self) -> Result<Value, String> {
        match self.current_token {
            Token::NUMBER(n) => {
                self.advance();
                Ok(Value::Number(n))
            }
            Token::STRING(s) => {
                self.advance();
                Ok(Value::String(s))
            }
            _ => Err("Expected value".to_string()),
        }
    }
    
    fn parse_set_clauses(&mut self) -> Result<Vec<(String, Value)>, String> {
        let mut set_clauses = Vec::new();
        
        loop {
            let column_name = match self.current_token {
                Token::IDENTIFIER(name) => {
                    self.advance();
                    name
                }
                _ => return Err("Expected column name".to_string()),
            };
            
            self.eat(Token::EQUAL)?;
            
            let value = self.parse_value()?;
            
            set_clauses.push((column_name, value));
            
            if self.current_token != Token::COMMA {
                break;
            }
            self.advance();
        }
        
        Ok(set_clauses)
    }
    
    fn parse_expr(&mut self) -> Result<Expr, String> {
        let left = self.parse_primary_expr()?;
        
        match self.current_token {
            Token::EQUAL => {
                self.advance();
                let right = self.parse_primary_expr()?;
                Ok(Expr::Equal(Box::new(left), Box::new(right)))
            }
            Token::LESS_THAN => {
                self.advance();
                let right = self.parse_primary_expr()?;
                Ok(Expr::LessThan(Box::new(left), Box::new(right)))
            }
            Token::GREATER_THAN => {
                self.advance();
                let right = self.parse_primary_expr()?;
                Ok(Expr::GreaterThan(Box::new(left), Box::new(right)))
            }
            _ => Ok(left),
        }
    }
    
    fn parse_primary_expr(&mut self) -> Result<Expr, String> {
        match self.current_token {
            Token::IDENTIFIER(name) => {
                self.advance();
                Ok(Expr::Column(name))
            }
            Token::NUMBER(n) => {
                self.advance();
                Ok(Expr::Value(Value::Number(n)))
            }
            Token::STRING(s) => {
                self.advance();
                Ok(Expr::Value(Value::String(s)))
            }
            _ => Err("Expected expression".to_string()),
        }
    }
    
    fn parse_column_definitions(&mut self) -> Result<Vec<ColumnDefinition>, String> {
        let mut columns = Vec::new();
        
        loop {
            let column_name = match self.current_token {
                Token::IDENTIFIER(name) => {
                    self.advance();
                    name
                }
                _ => return Err("Expected column name".to_string()),
            };
            
            let data_type = self.parse_data_type()?;
            
            columns.push(ColumnDefinition {
                name: column_name,
                data_type,
            });
            
            if self.current_token != Token::COMMA {
                break;
            }
            self.advance();
        }
        
        Ok(columns)
    }
    
    fn parse_data_type(&mut self) -> Result<DataType, String> {
        match self.current_token {
            Token::INT => {
                self.advance();
                Ok(DataType::Int)
            }
            Token::VARCHAR => {
                self.advance();
                self.eat(Token::LEFT_PAREN)?;
                let length = match self.current_token {
                    Token::NUMBER(n) => {
                        self.advance();
                        n as usize
                    }
                    _ => return Err("Expected length for VARCHAR".to_string()),
                };
                self.eat(Token::RIGHT_PAREN)?;
                Ok(DataType::Varchar(length))
            }
            _ => Err("Expected data type".to_string()),
        }
    }
    
    fn eat(&mut self, expected_token: Token) -> Result<(), String> {
        if self.current_token == expected_token {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, got {:?}", expected_token, self.current_token))
        }
    }
    
    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }
}

// ==================== 测试模块 ====================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== 1. SELECT语句解析测试 ====================

    #[test]
    fn test_parse_simple_select() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT * FROM users");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Select { table_name, columns, where_clause } => {
                assert_eq!(table_name, "users");
                assert_eq!(columns, vec!["*"]);
                assert!(where_clause.is_none());
            }
            _ => panic!("Expected Select statement"),
        }
    }

    #[test]
    fn test_parse_select_with_columns() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT id FROM users");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Select { table_name, columns, where_clause } => {
                assert_eq!(table_name, "users");
                assert_eq!(columns, vec!["id"]);
                assert!(where_clause.is_none());
            }
            _ => panic!("Expected Select statement"),
        }
    }

    #[test]
    fn test_parse_select_with_multiple_columns() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT id, name, email FROM users");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Select { table_name, columns, where_clause } => {
                assert_eq!(table_name, "users");
                assert_eq!(columns, vec!["id", "name", "email"]);
                assert!(where_clause.is_none());
            }
            _ => panic!("Expected Select statement"),
        }
    }

    #[test]
    fn test_parse_select_with_where() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT * FROM users WHERE id = 1");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Select { table_name, columns, where_clause } => {
                assert_eq!(table_name, "users");
                assert_eq!(columns, vec!["*"]);
                assert!(where_clause.is_some());
            }
            _ => panic!("Expected Select statement"),
        }
    }

    #[test]
    fn test_parse_select_with_where_greater_than() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT * FROM users WHERE age > 18");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Select { table_name, columns, where_clause } => {
                assert_eq!(table_name, "users");
                assert!(where_clause.is_some());
                if let Expr::GreaterThan(_, _) = where_clause.unwrap() {
                    // Expected
                } else {
                    panic!("Expected GreaterThan expression");
                }
            }
            _ => panic!("Expected Select statement"),
        }
    }

    #[test]
    fn test_parse_select_with_where_less_than() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT * FROM users WHERE score < 60");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Select { table_name, columns, where_clause } => {
                assert_eq!(table_name, "users");
                assert!(where_clause.is_some());
                if let Expr::LessThan(_, _) = where_clause.unwrap() {
                    // Expected
                } else {
                    panic!("Expected LessThan expression");
                }
            }
            _ => panic!("Expected Select statement"),
        }
    }

    #[test]
    fn test_parse_select_complex_where() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT id, name FROM users WHERE age > 18");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Select { table_name, columns, where_clause } => {
                assert_eq!(table_name, "users");
                assert_eq!(columns, vec!["id", "name"]);
                assert!(where_clause.is_some());
            }
            _ => panic!("Expected Select statement"),
        }
    }

    // ==================== 2. INSERT语句解析测试 ====================

    #[test]
    fn test_parse_insert_simple() {
        let mut parser = Parser::new();
        let result = parser.parse("INSERT INTO users VALUES (1, 'John')");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Insert { table_name, values } => {
                assert_eq!(table_name, "users");
                assert_eq!(values.len(), 2);
            }
            _ => panic!("Expected Insert statement"),
        }
    }

    #[test]
    fn test_parse_insert_with_single_value() {
        let mut parser = Parser::new();
        let result = parser.parse("INSERT INTO users VALUES (42)");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Insert { table_name, values } => {
                assert_eq!(table_name, "users");
                assert_eq!(values.len(), 1);
            }
            _ => panic!("Expected Insert statement"),
        }
    }

    #[test]
    fn test_parse_insert_with_multiple_values() {
        let mut parser = Parser::new();
        let result = parser.parse("INSERT INTO users VALUES (1, 'Alice', 25)");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Insert { table_name, values } => {
                assert_eq!(table_name, "users");
                assert_eq!(values.len(), 3);
            }
            _ => panic!("Expected Insert statement"),
        }
    }

    #[test]
    fn test_parse_insert_with_string_value() {
        let mut parser = Parser::new();
        let result = parser.parse("INSERT INTO products VALUES (1, 'Laptop')");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Insert { table_name, values } => {
                assert_eq!(table_name, "products");
                assert_eq!(values.len(), 2);
            }
            _ => panic!("Expected Insert statement"),
        }
    }

    #[test]
    fn test_parse_insert_multiple_rows() {
        let mut parser = Parser::new();
        let result = parser.parse("INSERT INTO users VALUES (1, 'John')");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Insert { table_name, values } => {
                assert_eq!(table_name, "users");
                assert!(!values.is_empty());
            }
            _ => panic!("Expected Insert statement"),
        }
    }

    // ==================== 3. UPDATE语句解析测试 ====================

    #[test]
    fn test_parse_update_simple() {
        let mut parser = Parser::new();
        let result = parser.parse("UPDATE users SET name = 'Jane'");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Update { table_name, set_clauses, where_clause } => {
                assert_eq!(table_name, "users");
                assert_eq!(set_clauses.len(), 1);
                assert!(where_clause.is_none());
            }
            _ => panic!("Expected Update statement"),
        }
    }

    #[test]
    fn test_parse_update_with_where() {
        let mut parser = Parser::new();
        let result = parser.parse("UPDATE users SET name = 'Jane' WHERE id = 1");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Update { table_name, set_clauses, where_clause } => {
                assert_eq!(table_name, "users");
                assert_eq!(set_clauses.len(), 1);
                assert!(where_clause.is_some());
            }
            _ => panic!("Expected Update statement"),
        }
    }

    #[test]
    fn test_parse_update_with_multiple_set() {
        let mut parser = Parser::new();
        let result = parser.parse("UPDATE users SET name = 'Jane', age = 30");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Update { table_name, set_clauses, where_clause } => {
                assert_eq!(table_name, "users");
                assert_eq!(set_clauses.len(), 2);
                assert!(where_clause.is_none());
            }
            _ => panic!("Expected Update statement"),
        }
    }

    #[test]
    fn test_parse_update_with_string_value() {
        let mut parser = Parser::new();
        let result = parser.parse("UPDATE users SET email = 'new@example.com' WHERE id = 1");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Update { table_name, set_clauses, where_clause } => {
                assert_eq!(table_name, "users");
                assert!(where_clause.is_some());
            }
            _ => panic!("Expected Update statement"),
        }
    }

    #[test]
    fn test_parse_update_with_number_value() {
        let mut parser = Parser::new();
        let result = parser.parse("UPDATE products SET price = 999 WHERE id = 1");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Update { table_name, set_clauses, where_clause } => {
                assert_eq!(table_name, "products");
                assert_eq!(set_clauses.len(), 1);
                assert!(where_clause.is_some());
            }
            _ => panic!("Expected Update statement"),
        }
    }

    // ==================== 4. DELETE语句解析测试 ====================

    #[test]
    fn test_parse_delete_all() {
        let mut parser = Parser::new();
        let result = parser.parse("DELETE FROM users");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Delete { table_name, where_clause } => {
                assert_eq!(table_name, "users");
                assert!(where_clause.is_none());
            }
            _ => panic!("Expected Delete statement"),
        }
    }

    #[test]
    fn test_parse_delete_with_where() {
        let mut parser = Parser::new();
        let result = parser.parse("DELETE FROM users WHERE id = 1");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Delete { table_name, where_clause } => {
                assert_eq!(table_name, "users");
                assert!(where_clause.is_some());
            }
            _ => panic!("Expected Delete statement"),
        }
    }

    #[test]
    fn test_parse_delete_with_complex_where() {
        let mut parser = Parser::new();
        let result = parser.parse("DELETE FROM users WHERE age > 18 AND status = 'inactive'");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Delete { table_name, where_clause } => {
                assert_eq!(table_name, "users");
                assert!(where_clause.is_some());
            }
            _ => panic!("Expected Delete statement"),
        }
    }

    #[test]
    fn test_parse_delete_different_table() {
        let mut parser = Parser::new();
        let result = parser.parse("DELETE FROM products WHERE stock = 0");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Delete { table_name, where_clause } => {
                assert_eq!(table_name, "products");
                assert!(where_clause.is_some());
            }
            _ => panic!("Expected Delete statement"),
        }
    }

    // ==================== 5. CREATE TABLE语句解析测试 ====================

    #[test]
    fn test_parse_create_table_simple() {
        let mut parser = Parser::new();
        let result = parser.parse("CREATE TABLE users (id INT)");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::CreateTable { table_name, columns } => {
                assert_eq!(table_name, "users");
                assert_eq!(columns.len(), 1);
                assert_eq!(columns[0].name, "id");
            }
            _ => panic!("Expected CreateTable statement"),
        }
    }

    #[test]
    fn test_parse_create_table_multiple_columns() {
        let mut parser = Parser::new();
        let result = parser.parse("CREATE TABLE users (id INT, name VARCHAR(255), age INT)");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::CreateTable { table_name, columns } => {
                assert_eq!(table_name, "users");
                assert_eq!(columns.len(), 3);
                assert_eq!(columns[0].name, "id");
                assert_eq!(columns[1].name, "name");
                assert_eq!(columns[2].name, "age");
            }
            _ => panic!("Expected CreateTable statement"),
        }
    }

    #[test]
    fn test_parse_create_table_with_varchar() {
        let mut parser = Parser::new();
        let result = parser.parse("CREATE TABLE users (name VARCHAR(100))");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::CreateTable { table_name, columns } => {
                assert_eq!(table_name, "users");
                assert_eq!(columns.len(), 1);
            }
            _ => panic!("Expected CreateTable statement"),
        }
    }

    #[test]
    fn test_parse_create_table_two_columns() {
        let mut parser = Parser::new();
        let result = parser.parse("CREATE TABLE products (id INT, name VARCHAR(255))");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::CreateTable { table_name, columns } => {
                assert_eq!(table_name, "products");
                assert_eq!(columns.len(), 2);
            }
            _ => panic!("Expected CreateTable statement"),
        }
    }

    // ==================== 6. 错误语法检测测试 ====================

    #[test]
    fn test_parse_error_empty_input() {
        let mut parser = Parser::new();
        let result = parser.parse("");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_invalid_keyword() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT * FROM users EXTRA");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_missing_from() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT * users");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_missing_table_name() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT * FROM");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_missing_value() {
        let mut parser = Parser::new();
        let result = parser.parse("INSERT INTO users VALUES");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_missing_set() {
        let mut parser = Parser::new();
        let result = parser.parse("UPDATE users name = 'test'");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_missing_where_column() {
        let mut parser = Parser::new();
        let result = parser.parse("DELETE FROM users WHERE");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_unexpected_token() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT * * FROM users");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_invalid_select_syntax() {
        let mut parser = Parser::new();
        let result = parser.parse("FROM users SELECT *");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_missing_parenthesis() {
        let mut parser = Parser::new();
        let result = parser.parse("INSERT INTO users VALUES (1, 2");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_missing_column_definition() {
        let mut parser = Parser::new();
        let result = parser.parse("CREATE TABLE users ()");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_invalid_table_name() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT * FROM 123");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_mismatched_parentheses() {
        let mut parser = Parser::new();
        let result = parser.parse("INSERT INTO users VALUES ((1, 2)");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_invalid_expression() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT * FROM users WHERE id =");
        assert!(result.is_err());
    }

    // ==================== 7. 边界条件测试 ====================

    #[test]
    fn test_parse_with_extra_whitespace() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT   *    FROM   users");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_with_newlines() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT *\nFROM users");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_with_tabs() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT\t*\tFROM users");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_long_table_name() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT * FROM very_long_table_name_123");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_long_column_name() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT very_long_column_name FROM users");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_multiple_statements() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT * FROM users");
        assert!(result.is_ok());
        
        let result2 = parser.parse("SELECT * FROM products");
        assert!(result2.is_ok());
    }

    // ==================== 8. 表达式解析测试 ====================

    #[test]
    fn test_parse_expression_equal() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT * FROM users WHERE id = 1");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Select { where_clause, .. } => {
                assert!(where_clause.is_some());
                if let Expr::Equal(_, _) = where_clause.unwrap() {
                    // Expected
                } else {
                    panic!("Expected Equal expression");
                }
            }
            _ => panic!("Expected Select statement"),
        }
    }

    #[test]
    fn test_parse_expression_greater_than() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT * FROM users WHERE age > 18");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Select { where_clause, .. } => {
                assert!(where_clause.is_some());
                if let Expr::GreaterThan(_, _) = where_clause.unwrap() {
                    // Expected
                } else {
                    panic!("Expected GreaterThan expression");
                }
            }
            _ => panic!("Expected Select statement"),
        }
    }

    #[test]
    fn test_parse_expression_less_than() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT * FROM users WHERE score < 60");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Select { where_clause, .. } => {
                assert!(where_clause.is_some());
                if let Expr::LessThan(_, _) = where_clause.unwrap() {
                    // Expected
                } else {
                    panic!("Expected LessThan expression");
                }
            }
            _ => panic!("Expected Select statement"),
        }
    }

    #[test]
    fn test_parse_expression_with_column() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT * FROM users WHERE id = user_id");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Select { where_clause, .. } => {
                assert!(where_clause.is_some());
            }
            _ => panic!("Expected Select statement"),
        }
    }

    #[test]
    fn test_parse_expression_with_string() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT * FROM users WHERE name = 'Alice'");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Select { where_clause, .. } => {
                assert!(where_clause.is_some());
            }
            _ => panic!("Expected Select statement"),
        }
    }

    // ==================== 9. 数据类型测试 ====================

    #[test]
    fn test_parse_data_type_int() {
        let mut parser = Parser::new();
        let result = parser.parse("CREATE TABLE test (id INT)");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::CreateTable { columns, .. } => {
                assert_eq!(columns.len(), 1);
                match columns[0].data_type {
                    DataType::Int => {}
                    _ => panic!("Expected Int data type"),
                }
            }
            _ => panic!("Expected CreateTable statement"),
        }
    }

    #[test]
    fn test_parse_data_type_varchar() {
        let mut parser = Parser::new();
        let result = parser.parse("CREATE TABLE test (name VARCHAR(255))");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::CreateTable { columns, .. } => {
                assert_eq!(columns.len(), 1);
                match columns[0].data_type {
                    DataType::Varchar(length) => {
                        assert_eq!(length, 255);
                    }
                    _ => panic!("Expected Varchar data type"),
                }
            }
            _ => panic!("Expected CreateTable statement"),
        }
    }

    #[test]
    fn test_parse_data_type_varchar_different_length() {
        let mut parser = Parser::new();
        let result = parser.parse("CREATE TABLE test (email VARCHAR(100))");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::CreateTable { columns, .. } => {
                match columns[0].data_type {
                    DataType::Varchar(length) => {
                        assert_eq!(length, 100);
                    }
                    _ => panic!("Expected Varchar data type"),
                }
            }
            _ => panic!("Expected CreateTable statement"),
        }
    }

    // ==================== 10. 值解析测试 ====================

    #[test]
    fn test_parse_value_number() {
        let mut parser = Parser::new();
        let result = parser.parse("INSERT INTO users VALUES (42)");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Insert { values, .. } => {
                assert_eq!(values.len(), 1);
                match &values[0] {
                    Value::Number(n) => assert_eq!(*n, 42),
                    _ => panic!("Expected Number value"),
                }
            }
            _ => panic!("Expected Insert statement"),
        }
    }

    #[test]
    fn test_parse_value_string() {
        let mut parser = Parser::new();
        let result = parser.parse("INSERT INTO users VALUES ('Hello World')");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Insert { values, .. } => {
                match &values[0] {
                    Value::String(s) => assert_eq!(s, "Hello World"),
                    _ => panic!("Expected String value"),
                }
            }
            _ => panic!("Expected Insert statement"),
        }
    }

    #[test]
    fn test_parse_value_multiple() {
        let mut parser = Parser::new();
        let result = parser.parse("INSERT INTO users VALUES (1, 'John', 25)");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Insert { values, .. } => {
                assert_eq!(values.len(), 3);
            }
            _ => panic!("Expected Insert statement"),
        }
    }

    // ==================== 11. SET子句解析测试 ====================

    #[test]
    fn test_parse_set_clause_single() {
        let mut parser = Parser::new();
        let result = parser.parse("UPDATE users SET name = 'Alice'");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Update { set_clauses, .. } => {
                assert_eq!(set_clauses.len(), 1);
                assert_eq!(set_clauses[0].0, "name");
            }
            _ => panic!("Expected Update statement"),
        }
    }

    #[test]
    fn test_parse_set_clause_multiple() {
        let mut parser = Parser::new();
        let result = parser.parse("UPDATE users SET name = 'Alice', age = 30");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Update { set_clauses, .. } => {
                assert_eq!(set_clauses.len(), 2);
                assert_eq!(set_clauses[0].0, "name");
                assert_eq!(set_clauses[1].0, "age");
            }
            _ => panic!("Expected Update statement"),
        }
    }

    #[test]
    fn test_parse_set_clause_with_number() {
        let mut parser = Parser::new();
        let result = parser.parse("UPDATE users SET age = 25");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Update { set_clauses, .. } => {
                assert_eq!(set_clauses.len(), 1);
                match &set_clauses[0].1 {
                    Value::Number(n) => assert_eq!(*n, 25),
                    _ => panic!("Expected Number value"),
                }
            }
            _ => panic!("Expected Update statement"),
        }
    }

    #[test]
    fn test_parse_set_clause_with_string() {
        let mut parser = Parser::new();
        let result = parser.parse("UPDATE users SET status = 'active'");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Update { set_clauses, .. } => {
                assert_eq!(set_clauses.len(), 1);
                match &set_clauses[0].1 {
                    Value::String(s) => assert_eq!(s, "active"),
                    _ => panic!("Expected String value"),
                }
            }
            _ => panic!("Expected Update statement"),
        }
    }

    // ==================== 12. 扩展INSERT测试 ====================

    #[test]
    fn test_parse_insert_different_table() {
        let mut parser = Parser::new();
        let result = parser.parse("INSERT INTO products VALUES (1, 'Laptop', 999.99)");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Insert { table_name, values } => {
                assert_eq!(table_name, "products");
                assert_eq!(values.len(), 3);
            }
            _ => panic!("Expected Insert statement"),
        }
    }

    #[test]
    fn test_parse_insert_large_number() {
        let mut parser = Parser::new();
        let result = parser.parse("INSERT INTO stats VALUES (1234567890)");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Insert { values, .. } => {
                match &values[0] {
                    Value::Number(n) => assert_eq!(*n, 1234567890),
                    _ => panic!("Expected Number value"),
                }
            }
            _ => panic!("Expected Insert statement"),
        }
    }

    #[test]
    fn test_parse_insert_negative_number() {
        let mut parser = Parser::new();
        let result = parser.parse("INSERT INTO temperatures VALUES (-10)");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Insert { values, .. } => {
                match &values[0] {
                    Value::Number(n) => assert_eq!(*n, -10),
                    _ => panic!("Expected Number value"),
                }
            }
            _ => panic!("Expected Insert statement"),
        }
    }

    #[test]
    fn test_parse_insert_with_special_characters() {
        let mut parser = Parser::new();
        let result = parser.parse("INSERT INTO logs VALUES ('Error: null pointer')");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Insert { values, .. } => {
                match &values[0] {
                    Value::String(s) => assert_eq!(s, "Error: null pointer"),
                    _ => panic!("Expected String value"),
                }
            }
            _ => panic!("Expected Insert statement"),
        }
    }

    // ==================== 13. 扩展UPDATE测试 ====================

    #[test]
    fn test_parse_update_without_where() {
        let mut parser = Parser::new();
        let result = parser.parse("UPDATE users SET active = 1");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Update { table_name, set_clauses, where_clause } => {
                assert_eq!(table_name, "users");
                assert_eq!(set_clauses.len(), 1);
                assert!(where_clause.is_none());
            }
            _ => panic!("Expected Update statement"),
        }
    }

    #[test]
    fn test_parse_update_multiple_columns() {
        let mut parser = Parser::new();
        let result = parser.parse("UPDATE users SET name = 'Bob', age = 25, email = 'bob@test.com', active = 1 WHERE id = 5");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Update { set_clauses, where_clause, .. } => {
                assert_eq!(set_clauses.len(), 4);
                assert!(where_clause.is_some());
            }
            _ => panic!("Expected Update statement"),
        }
    }

    #[test]
    fn test_parse_update_with_greater_condition() {
        let mut parser = Parser::new();
        let result = parser.parse("UPDATE products SET price = price * 0.9 WHERE stock > 100");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Update { where_clause, .. } => {
                assert!(where_clause.is_some());
            }
            _ => panic!("Expected Update statement"),
        }
    }

    #[test]
    fn test_parse_update_with_less_condition() {
        let mut parser = Parser::new();
        let result = parser.parse("UPDATE users SET status = 'inactive' WHERE login_count < 5");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Update { where_clause, .. } => {
                assert!(where_clause.is_some());
            }
            _ => panic!("Expected Update statement"),
        }
    }

    // ==================== 14. 扩展DELETE测试 ====================

    #[test]
    fn test_parse_delete_without_where() {
        let mut parser = Parser::new();
        let result = parser.parse("DELETE FROM temp_data");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Delete { table_name, where_clause } => {
                assert_eq!(table_name, "temp_data");
                assert!(where_clause.is_none());
            }
            _ => panic!("Expected Delete statement"),
        }
    }

    #[test]
    fn test_parse_delete_with_greater_condition() {
        let mut parser = Parser::new();
        let result = parser.parse("DELETE FROM logs WHERE created_at > '2024-01-01'");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Delete { where_clause, .. } => {
                assert!(where_clause.is_some());
            }
            _ => panic!("Expected Delete statement"),
        }
    }

    #[test]
    fn test_parse_delete_with_less_condition() {
        let mut parser = Parser::new();
        let result = parser.parse("DELETE FROM cache WHERE expires < 1234567890");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Delete { where_clause, .. } => {
                assert!(where_clause.is_some());
            }
            _ => panic!("Expected Delete statement"),
        }
    }

    #[test]
    fn test_parse_delete_with_not_equal() {
        let mut parser = Parser::new();
        let result = parser.parse("DELETE FROM users WHERE status <> 'deleted'");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Delete { where_clause, .. } => {
                assert!(where_clause.is_some());
            }
            _ => panic!("Expected Delete statement"),
        }
    }

    // ==================== 15. 扩展CREATE TABLE测试 ====================

    #[test]
    fn test_parse_create_table_with_many_columns() {
        let mut parser = Parser::new();
        let result = parser.parse("CREATE TABLE users (id INT, name VARCHAR(255), email VARCHAR(100), age INT, created_at VARCHAR(50), active INT)");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::CreateTable { columns, .. } => {
                assert_eq!(columns.len(), 6);
            }
            _ => panic!("Expected CreateTable statement"),
        }
    }

    #[test]
    fn test_parse_create_table_with_only_varchar() {
        let mut parser = Parser::new();
        let result = parser.parse("CREATE TABLE configs (key VARCHAR(100), value VARCHAR(255))");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::CreateTable { columns, .. } => {
                assert_eq!(columns.len(), 2);
            }
            _ => panic!("Expected CreateTable statement"),
        }
    }

    #[test]
    fn test_parse_create_table_with_only_int() {
        let mut parser = Parser::new();
        let result = parser.parse("CREATE TABLE counts (a INT, b INT, c INT)");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::CreateTable { columns, .. } => {
                assert_eq!(columns.len(), 3);
            }
            _ => panic!("Expected CreateTable statement"),
        }
    }

    // ==================== 16. 扩展错误检测测试 ====================

    #[test]
    fn test_parse_error_incomplete_insert() {
        let mut parser = Parser::new();
        let result = parser.parse("INSERT INTO");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_incomplete_update() {
        let mut parser = Parser::new();
        let result = parser.parse("UPDATE");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_incomplete_delete() {
        let mut parser = Parser::new();
        let result = parser.parse("DELETE");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_incomplete_create_table() {
        let mut parser = Parser::new();
        let result = parser.parse("CREATE");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_wrong_keyword_order() {
        let mut parser = Parser::new();
        let result = parser.parse("users FROM SELECT *");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_missing_table_in_insert() {
        let mut parser = Parser::new();
        let result = parser.parse("INSERT INTO VALUES (1)");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_missing_table_in_update() {
        let mut parser = Parser::new();
        let result = parser.parse("UPDATE SET name = 'test'");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_missing_table_in_delete() {
        let mut parser = Parser::new();
        let result = parser.parse("DELETE WHERE id = 1");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_invalid_column_in_select() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT FROM users");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_invalid_expression_right() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT * FROM users WHERE id >");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_invalid_set_clause() {
        let mut parser = Parser::new();
        let result = parser.parse("UPDATE users SET");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_invalid_value_in_insert() {
        let mut parser = Parser::new();
        let result = parser.parse("INSERT INTO users VALUES ()");
        assert!(result.is_err());
    }

    // ==================== 17. 表达式扩展测试 ====================

    #[test]
    fn test_parse_expression_not_equal() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT * FROM users WHERE status != 'inactive'");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Select { where_clause, .. } => {
                assert!(where_clause.is_some());
            }
            _ => panic!("Expected Select statement"),
        }
    }

    #[test]
    fn test_parse_expression_greater_than_or_equal() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT * FROM users WHERE age >= 18");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Select { where_clause, .. } => {
                assert!(where_clause.is_some());
            }
            _ => panic!("Expected Select statement"),
        }
    }

    #[test]
    fn test_parse_expression_less_than_or_equal() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT * FROM users WHERE score <= 100");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Select { where_clause, .. } => {
                assert!(where_clause.is_some());
            }
            _ => panic!("Expected Select statement"),
        }
    }

    // ==================== 18. 边界条件扩展测试 ====================

    #[test]
    fn test_parse_with_multiple_spaces() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT    *    FROM    users");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_with_carriage_return() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT *\rFROM users");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_with_mixed_line_endings() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT *\r\nFROM users\n");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_empty_table_name() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT * FROM");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_with_semicolon() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT * FROM users;");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_with_semicolon_and_space() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT * FROM users ; ");
        assert!(result.is_ok());
    }

    // ==================== 19. 连续语句测试 ====================

    #[test]
    fn test_parse_sequential_selects() {
        let mut parser = Parser::new();
        
        let result1 = parser.parse("SELECT * FROM users");
        assert!(result1.is_ok());
        
        let result2 = parser.parse("SELECT * FROM products");
        assert!(result2.is_ok());
        
        let result3 = parser.parse("SELECT * FROM orders");
        assert!(result3.is_ok());
    }

    #[test]
    fn test_parse_sequential_inserts() {
        let mut parser = Parser::new();
        
        let result1 = parser.parse("INSERT INTO users VALUES (1, 'Alice')");
        assert!(result1.is_ok());
        
        let result2 = parser.parse("INSERT INTO users VALUES (2, 'Bob')");
        assert!(result2.is_ok());
    }

    #[test]
    fn test_parse_mixed_statements() {
        let mut parser = Parser::new();
        
        let result1 = parser.parse("SELECT * FROM users");
        assert!(result1.is_ok());
        
        let result2 = parser.parse("INSERT INTO users VALUES (1, 'Test')");
        assert!(result2.is_ok());
        
        let result3 = parser.parse("UPDATE users SET name = 'New' WHERE id = 1");
        assert!(result3.is_ok());
        
        let result4 = parser.parse("DELETE FROM users WHERE id = 2");
        assert!(result4.is_ok());
    }

    // ==================== 20. 特定场景测试 ====================

    #[test]
    fn test_parse_select_with_single_column() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT id FROM users");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Select { columns, .. } => {
                assert_eq!(columns.len(), 1);
                assert_eq!(columns[0], "id");
            }
            _ => panic!("Expected Select statement"),
        }
    }

    #[test]
    fn test_parse_insert_with_five_values() {
        let mut parser = Parser::new();
        let result = parser.parse("INSERT INTO records VALUES (1, 2, 3, 4, 5)");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Insert { values, .. } => {
                assert_eq!(values.len(), 5);
            }
            _ => panic!("Expected Insert statement"),
        }
    }

    #[test]
    fn test_parse_update_with_single_set() {
        let mut parser = Parser::new();
        let result = parser.parse("UPDATE config SET value = 'new_value'");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Update { set_clauses, .. } => {
                assert_eq!(set_clauses.len(), 1);
            }
            _ => panic!("Expected Update statement"),
        }
    }

    #[test]
    fn test_parse_delete_with_string_comparison() {
        let mut parser = Parser::new();
        let result = parser.parse("DELETE FROM users WHERE name = 'John'");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::Delete { where_clause, .. } => {
                assert!(where_clause.is_some());
            }
            _ => panic!("Expected Delete statement"),
        }
    }

    #[test]
    fn test_parse_create_table_with_two_columns() {
        let mut parser = Parser::new();
        let result = parser.parse("CREATE TABLE sessions (id INT, token VARCHAR(255))");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt {
            Statement::CreateTable { columns, .. } => {
                assert_eq!(columns.len(), 2);
            }
            _ => panic!("Expected CreateTable statement"),
        }
    }

    // ==================== 21. AST节点验证测试 ====================

    #[test]
    fn test_ast_select_structure() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT a, b FROM t WHERE c = 1");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        
        if let Statement::Select { table_name, columns, where_clause } = stmt {
            assert_eq!(table_name, "t");
            assert_eq!(columns.len(), 2);
            assert!(where_clause.is_some());
        } else {
            panic!("Expected Select statement");
        }
    }

    #[test]
    fn test_ast_insert_structure() {
        let mut parser = Parser::new();
        let result = parser.parse("INSERT INTO users VALUES (1, 'test')");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        
        if let Statement::Insert { table_name, values } = stmt {
            assert_eq!(table_name, "users");
            assert_eq!(values.len(), 2);
        } else {
            panic!("Expected Insert statement");
        }
    }

    #[test]
    fn test_ast_update_structure() {
        let mut parser = Parser::new();
        let result = parser.parse("UPDATE users SET name = 'new' WHERE id = 1");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        
        if let Statement::Update { table_name, set_clauses, where_clause } = stmt {
            assert_eq!(table_name, "users");
            assert_eq!(set_clauses.len(), 1);
            assert!(where_clause.is_some());
        } else {
            panic!("Expected Update statement");
        }
    }

    #[test]
    fn test_ast_delete_structure() {
        let mut parser = Parser::new();
        let result = parser.parse("DELETE FROM users WHERE id = 1");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        
        if let Statement::Delete { table_name, where_clause } = stmt {
            assert_eq!(table_name, "users");
            assert!(where_clause.is_some());
        } else {
            panic!("Expected Delete statement");
        }
    }

    #[test]
    fn test_ast_create_table_structure() {
        let mut parser = Parser::new();
        let result = parser.parse("CREATE TABLE t (a INT, b VARCHAR(10))");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        
        if let Statement::CreateTable { table_name, columns } = stmt {
            assert_eq!(table_name, "t");
            assert_eq!(columns.len(), 2);
        } else {
            panic!("Expected CreateTable statement");
        }
    }
}
