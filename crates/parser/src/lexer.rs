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

// ==================== 扩展测试模块 ====================

#[cfg(test)]
mod lexer_extended_tests {
    use super::*;

    // ==================== 1. 扩展运算符测试 ====================

    #[test]
    fn test_lexer_not_equal() {
        let mut lexer = Lexer::new("!=");
        let token = lexer.next_token();
        assert_eq!(token, Token::NotEqual);
    }

    #[test]
    fn test_lexer_less_equal() {
        let mut lexer = Lexer::new("<=");
        let token = lexer.next_token();
        assert_eq!(token, Token::LessEqual);
    }

    #[test]
    fn test_lexer_greater_equal() {
        let mut lexer = Lexer::new(">=");
        let token = lexer.next_token();
        assert_eq!(token, Token::GreaterEqual);
    }

    #[test]
    fn test_lexer_not_equal_alt() {
        let mut lexer = Lexer::new("<>");
        let token = lexer.next_token();
        assert_eq!(token, Token::NotEqual);
    }

    #[test]
    fn test_lexer_multiple_operators() {
        let mut lexer = Lexer::new("= <> != > >= < <=");
        assert_eq!(lexer.next_token(), Token::Equal);
        assert_eq!(lexer.next_token(), Token::NotEqual);
        assert_eq!(lexer.next_token(), Token::NotEqual);
        assert_eq!(lexer.next_token(), Token::Greater);
        assert_eq!(lexer.next_token(), Token::GreaterEqual);
        assert_eq!(lexer.next_token(), Token::Less);
        assert_eq!(lexer.next_token(), Token::LessEqual);
    }

    // ==================== 2. 扩展标点符号测试 ====================

    #[test]
    fn test_lexer_all_punctuation() {
        let mut lexer = Lexer::new("( ) , ;");
        assert_eq!(lexer.next_token(), Token::LeftParen);
        assert_eq!(lexer.next_token(), Token::RightParen);
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::Semicolon);
    }

    #[test]
    fn test_lexer_dot() {
        let mut lexer = Lexer::new(".");
        assert_eq!(lexer.next_token(), Token::Dot);
    }

    #[test]
    fn test_lexer_colon() {
        let mut lexer = Lexer::new(":");
        assert_eq!(lexer.next_token(), Token::Colon);
    }

    #[test]
    fn test_lexer_star() {
        let mut lexer = Lexer::new("*");
        assert_eq!(lexer.next_token(), Token::Star);
    }

    #[test]
    fn test_lexer_slash() {
        let mut lexer = Lexer::new("/");
        assert_eq!(lexer.next_token(), Token::Slash);
    }

    #[test]
    fn test_lexer_percent() {
        let mut lexer = Lexer::new("%");
        assert_eq!(lexer.next_token(), Token::Percent);
    }

    #[test]
    fn test_lexer_plus() {
        let mut lexer = Lexer::new("+");
        assert_eq!(lexer.next_token(), Token::Plus);
    }

    #[test]
    fn test_lexer_minus() {
        let mut lexer = Lexer::new("-");
        assert_eq!(lexer.next_token(), Token::Minus);
    }

    // ==================== 3. 扩展标识符测试 ====================

    #[test]
    fn test_lexer_identifier_with_numbers_middle() {
        let tokens = tokenize("user123name");
        assert_eq!(tokens[0], Token::Identifier("user123name".to_string()));
    }

    #[test]
    fn test_lexer_identifier_with_multiple_underscores() {
        let tokens = tokenize("user__name__test");
        assert_eq!(tokens[0], Token::Identifier("user__name__test".to_string()));
    }

    #[test]
    fn test_lexer_identifier_starting_with_number() {
        let tokens = tokenize("123users");
        assert_eq!(tokens[0], Token::Identifier("123users".to_string()));
    }

    #[test]
    fn test_lexer_multiple_identifiers() {
        let tokens = tokenize("table1, table2, table3");
        match &tokens[0] {
            Token::Identifier(name) => assert_eq!(name, "table1"),
            _ => panic!("Expected Identifier"),
        }
        assert_eq!(tokens[1], Token::Comma);
        match &tokens[2] {
            Token::Identifier(name) => assert_eq!(name, "table2"),
            _ => panic!("Expected Identifier"),
        }
        assert_eq!(tokens[3], Token::Comma);
        match &tokens[4] {
            Token::Identifier(name) => assert_eq!(name, "table3"),
            _ => panic!("Expected Identifier"),
        }
    }

    #[test]
    fn test_lexer_identifier_preserves_case() {
        let tokens = tokenize("UserName TABLE_NAME Test123");
        assert_eq!(tokens[0], Token::Identifier("UserName".to_string()));
        assert_eq!(tokens[1], Token::Identifier("TABLE_NAME".to_string()));
        assert_eq!(tokens[2], Token::Identifier("Test123".to_string()));
    }

    // ==================== 4. 扩展数字测试 ====================

    #[test]
    fn test_lexer_large_number() {
        let tokens = tokenize("1234567890");
        match &tokens[0] {
            Token::Number(n) => assert_eq!(*n, 1234567890),
            _ => panic!("Expected Number"),
        }
    }

    #[test]
    fn test_lexer_negative_number() {
        let tokens = tokenize("-100");
        assert_eq!(tokens[0], Token::Minus);
        match &tokens[1] {
            Token::Number(n) => assert_eq!(*n, 100),
            _ => panic!("Expected Number"),
        }
    }

    #[test]
    fn test_lexer_decimal_zero() {
        let tokens = tokenize("0.0");
        match &tokens[0] {
            Token::Number(n) => assert_eq!(*n, 0),
            _ => panic!("Expected Number"),
        }
    }

    #[test]
    fn test_lexer_multiple_decimals_in_expression() {
        let tokens = tokenize("1.5 + 2.5 - 3.5");
        match &tokens[0] {
            Token::Number(n) => assert_eq!(*n, 1),
            _ => panic!("Expected Number"),
        }
        assert_eq!(tokens[1], Token::Plus);
        match &tokens[2] {
            Token::Number(n) => assert_eq!(*n, 5),
            _ => panic!("Expected Number"),
        }
    }

    // ==================== 5. 扩展字符串测试 ====================

    #[test]
    fn test_lexer_string_with_single_quote_inside() {
        let tokens = tokenize("'it''s a test'");
        match &tokens[0] {
            Token::String(s) => assert_eq!(s, "it''s a test"),
            _ => panic!("Expected String"),
        }
    }

    #[test]
    fn test_lexer_string_empty_quotes() {
        let tokens = tokenize("''");
        match &tokens[0] {
            Token::String(s) => assert_eq!(s, ""),
            _ => panic!("Expected String"),
        }
    }

    #[test]
    fn test_lexer_string_with_spaces() {
        let tokens = tokenize("'hello world'");
        match &tokens[0] {
            Token::String(s) => assert_eq!(s, "hello world"),
            _ => panic!("Expected String"),
        }
    }

    #[test]
    fn test_lexer_string_with_unicode() {
        let tokens = tokenize("'你好世界'");
        match &tokens[0] {
            Token::String(s) => assert_eq!(s, "你好世界"),
            _ => panic!("Expected String"),
        }
    }

    #[test]
    fn test_lexer_string_with_special_chars() {
        let tokens = tokenize("'email: test@example.com'");
        match &tokens[0] {
            Token::String(s) => assert_eq!(s, "email: test@example.com"),
            _ => panic!("Expected String"),
        }
    }

    #[test]
    fn test_lexer_multiple_strings() {
        let tokens = tokenize("'first', 'second', 'third'");
        match &tokens[0] {
            Token::String(s) => assert_eq!(s, "first"),
            _ => panic!("Expected String"),
        }
        assert_eq!(tokens[1], Token::Comma);
        match &tokens[2] {
            Token::String(s) => assert_eq!(s, "second"),
            _ => panic!("Expected String"),
        }
        assert_eq!(tokens[3], Token::Comma);
        match &tokens[4] {
            Token::String(s) => assert_eq!(s, "third"),
            _ => panic!("Expected String"),
        }
    }

    // ==================== 6. 扩展关键字测试 ====================

    #[test]
    fn test_lexer_keyword_into() {
        let tokens = tokenize("INTO");
        assert_eq!(tokens[0], Token::Into);
    }

    #[test]
    fn test_lexer_keyword_values() {
        let tokens = tokenize("VALUES");
        assert_eq!(tokens[0], Token::Values);
    }

    #[test]
    fn test_lexer_keyword_set() {
        let tokens = tokenize("SET");
        assert_eq!(tokens[0], Token::Set);
    }

    #[test]
    fn test_lexer_keyword_drop() {
        let tokens = tokenize("DROP");
        assert_eq!(tokens[0], Token::Drop);
    }

    #[test]
    fn test_lexer_keyword_alter() {
        let tokens = tokenize("ALTER");
        assert_eq!(tokens[0], Token::Alter);
    }

    #[test]
    fn test_lexer_keyword_index() {
        let tokens = tokenize("INDEX");
        assert_eq!(tokens[0], Token::Index);
    }

    #[test]
    fn test_lexer_keyword_on() {
        let tokens = tokenize("ON");
        assert_eq!(tokens[0], Token::On);
    }

    #[test]
    fn test_lexer_keyword_primary() {
        let tokens = tokenize("PRIMARY");
        assert_eq!(tokens[0], Token::Primary);
    }

    #[test]
    fn test_lexer_keyword_key() {
        let tokens = tokenize("KEY");
        assert_eq!(tokens[0], Token::Key);
    }

    #[test]
    fn test_lexer_keyword_begin() {
        let tokens = tokenize("BEGIN");
        assert_eq!(tokens[0], Token::Begin);
    }

    #[test]
    fn test_lexer_keyword_commit() {
        let tokens = tokenize("COMMIT");
        assert_eq!(tokens[0], Token::Commit);
    }

    #[test]
    fn test_lexer_keyword_rollback() {
        let tokens = tokenize("ROLLBACK");
        assert_eq!(tokens[0], Token::Rollback);
    }

    #[test]
    fn test_lexer_keyword_grant() {
        let tokens = tokenize("GRANT");
        assert_eq!(tokens[0], Token::Grant);
    }

    #[test]
    fn test_lexer_keyword_revoke() {
        let tokens = tokenize("REVOKE");
        assert_eq!(tokens[0], Token::Revoke);
    }

    // ==================== 7. 扩展空白字符测试 ====================

    #[test]
    fn test_lexer_only_spaces() {
        let tokens = tokenize("     ");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::EOF);
    }

    #[test]
    fn test_lexer_only_tabs() {
        let tokens = tokenize("\t\t\t");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::EOF);
    }

    #[test]
    fn test_lexer_only_newlines() {
        let tokens = tokenize("\n\n\n");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::EOF);
    }

    #[test]
    fn test_lexer_mixed_whitespace() {
        let tokens = tokenize("  \t\n\r  ");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::EOF);
    }

    #[test]
    fn test_lexer_leading_and_trailing_whitespace() {
        let tokens = tokenize("   SELECT id   ");
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::Select);
        match &tokens[1] {
            Token::Identifier(name) => assert_eq!(name, "id"),
            _ => panic!("Expected Identifier"),
        }
        assert_eq!(tokens[2], Token::EOF);
    }

    #[test]
    fn test_lexer_whitespace_between_all_tokens() {
        let tokens = tokenize("SELECT   id   FROM   users   WHERE   id   =   1");
        assert_eq!(tokens.len(), 11);
        assert_eq!(tokens[0], Token::Select);
    }

    // ==================== 8. 复杂SQL语句测试 ====================

    #[test]
    fn test_lexer_complex_select_with_multiple_conditions() {
        let tokens = tokenize("SELECT id, name, email FROM users WHERE age > 18 AND status = 'active'");
        assert_eq!(tokens[0], Token::Select);
        assert_eq!(tokens[7], Token::From);
        assert_eq!(tokens[9], Token::Where);
    }

    #[test]
    fn test_lexer_insert_with_all_types() {
        let tokens = tokenize("INSERT INTO users VALUES (1, 'John', 25, 'john@example.com')");
        assert_eq!(tokens[0], Token::Insert);
        assert_eq!(tokens[1], Token::Into);
    }

    #[test]
    fn test_lexer_update_with_multiple_sets() {
        let tokens = tokenize("UPDATE users SET name = 'Jane', age = 30, email = 'jane@example.com' WHERE id = 1");
        assert_eq!(tokens[0], Token::Update);
        assert_eq!(tokens[2], Token::Set);
        assert_eq!(tokens[14], Token::Where);
    }

    #[test]
    fn test_lexer_delete_with_complex_condition() {
        let tokens = tokenize("DELETE FROM users WHERE age > 18 AND status = 'inactive' OR id = 999");
        assert_eq!(tokens[0], Token::Delete);
        assert_eq!(tokens[1], Token::From);
        assert_eq!(tokens[3], Token::Where);
    }

    #[test]
    fn test_lexer_create_table_with_constraints() {
        let tokens = tokenize("CREATE TABLE users (id INT PRIMARY KEY, name VARCHAR(255), age INT, email VARCHAR(100))");
        assert_eq!(tokens[0], Token::Create);
        assert_eq!(tokens[1], Token::Table);
    }

    // ==================== 9. 边界条件测试 ====================

    #[test]
    fn test_lexer_single_character_input() {
        let tokens = tokenize("a");
        match &tokens[0] {
            Token::Identifier(name) => assert_eq!(name, "a"),
            _ => panic!("Expected Identifier"),
        }
    }

    #[test]
    fn test_lexer_single_digit_input() {
        let tokens = tokenize("5");
        match &tokens[0] {
            Token::Number(n) => assert_eq!(*n, 5),
            _ => panic!("Expected Number"),
        }
    }

    #[test]
    fn test_lexer_single_quote_input() {
        let tokens = tokenize("'");
        match &tokens[0] {
            Token::String(s) => assert_eq!(s, ""),
            _ => panic!("Expected String"),
        }
    }

    #[test]
    fn test_lexer_unknown_character() {
        let tokens = tokenize("@");
        match &tokens[0] {
            Token::Identifier(name) => assert_eq!(name, "@"),
            _ => panic!("Expected Identifier"),
        }
    }

    #[test]
    fn test_lexer_multiple_unknown_characters() {
        let tokens = tokenize("@#$%");
        match &tokens[0] {
            Token::Identifier(name) => assert_eq!(name, "@"),
            _ => panic!("Expected Identifier"),
        }
    }

    // ==================== 10. Token位置测试 ====================

    #[test]
    fn test_lexer_position_tracking_simple() {
        let mut lexer = Lexer::new("SELECT");
        assert_eq!(lexer.position, 0);
        lexer.next_token();
        assert!(lexer.position > 0);
    }

    #[test]
    fn test_lexer_position_tracking_multiple() {
        let mut lexer = Lexer::new("SELECT * FROM");
        let pos1 = lexer.position;
        lexer.next_token(); // SELECT
        let pos2 = lexer.position;
        assert!(pos2 > pos1);
        lexer.next_token(); // *
        lexer.next_token(); // FROM
        let pos3 = lexer.position;
        assert!(pos3 > pos2);
    }

    #[test]
    fn test_lexer_eof_reached() {
        let mut lexer = Lexer::new("a");
        lexer.next_token();
        let token = lexer.next_token();
        assert_eq!(token, Token::EOF);
    }

    #[test]
    fn test_lexer_multiple_eof() {
        let mut lexer = Lexer::new("SELECT");
        lexer.next_token();
        let token1 = lexer.next_token();
        let token2 = lexer.next_token();
        assert_eq!(token1, Token::EOF);
        assert_eq!(token2, Token::EOF);
    }
}
