// 词法分析器

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // 关键字
    SELECT,
    INSERT,
    UPDATE,
    DELETE,
    CREATE,
    TABLE,
    FROM,
    WHERE,
    VALUES,
    SET,
    INT,
    VARCHAR,
    
    // 运算符
    EQUAL,
    LESS_THAN,
    GREATER_THAN,
    
    // 标点符号
    COMMA,
    SEMICOLON,
    LEFT_PAREN,
    RIGHT_PAREN,
    
    // 字面量
    NUMBER(i32),
    STRING(String),
    IDENTIFIER(String),
    
    // 结束
    EOF,
}

pub struct Lexer {
    input: String,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Self {
            input: input.to_string(),
            position: 0,
            current_char: None,
        };
        lexer.advance();
        lexer
    }
    
    fn advance(&mut self) {
        if self.position < self.input.len() {
            self.current_char = Some(self.input.chars().nth(self.position).unwrap());
            self.position += 1;
        } else {
            self.current_char = None;
        }
    }
    
    fn skip_whitespace(&mut self) {
        while self.current_char.is_some() && self.current_char.unwrap().is_whitespace() {
            self.advance();
        }
    }
    
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        
        match self.current_char {
            None => Token::EOF,
            Some('=') => {
                self.advance();
                Token::EQUAL
            }
            Some('<') => {
                self.advance();
                Token::LESS_THAN
            }
            Some('>') => {
                self.advance();
                Token::GREATER_THAN
            }
            Some(',') => {
                self.advance();
                Token::COMMA
            }
            Some(';') => {
                self.advance();
                Token::SEMICOLON
            }
            Some('(') => {
                self.advance();
                Token::LEFT_PAREN
            }
            Some(')') => {
                self.advance();
                Token::RIGHT_PAREN
            }
            Some('"') => {
                self.advance();
                let mut string = String::new();
                while self.current_char.is_some() && self.current_char.unwrap() != '"' {
                    string.push(self.current_char.unwrap());
                    self.advance();
                }
                if self.current_char.is_some() {
                    self.advance();
                }
                Token::STRING(string)
            }
            Some(c) if c.is_digit(10) => {
                let mut number = String::new();
                while self.current_char.is_some() && self.current_char.unwrap().is_digit(10) {
                    number.push(self.current_char.unwrap());
                    self.advance();
                }
                Token::NUMBER(number.parse().unwrap())
            }
            Some(c) if c.is_alphabetic() || c == '_' => {
                let mut identifier = String::new();
                while self.current_char.is_some() && 
                      (self.current_char.unwrap().is_alphanumeric() || self.current_char.unwrap() == '_') {
                    identifier.push(self.current_char.unwrap());
                    self.advance();
                }
                match identifier.to_uppercase().as_str() {
                    "SELECT" => Token::SELECT,
                    "INSERT" => Token::INSERT,
                    "UPDATE" => Token::UPDATE,
                    "DELETE" => Token::DELETE,
                    "CREATE" => Token::CREATE,
                    "TABLE" => Token::TABLE,
                    "FROM" => Token::FROM,
                    "WHERE" => Token::WHERE,
                    "VALUES" => Token::VALUES,
                    "SET" => Token::SET,
                    "INT" => Token::INT,
                    "VARCHAR" => Token::VARCHAR,
                    _ => Token::IDENTIFIER(identifier),
                }
            }
            _ => {
                self.advance();
                Token::EOF
            }
        }
    }
}
