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
