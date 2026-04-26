# Parser模块设计文档

## 1. 模块概述

Parser模块是SQLRustGo的核心组件之一，负责SQL语句的解析和处理。它将用户输入的SQL文本转换为抽象语法树（AST），为后续的查询优化和执行提供基础。

### 1.1 主要职责
- 词法分析：将SQL语句分解为词法单元（Token）
- 语法分析：根据SQL语法规则构建抽象语法树（AST）
- 语义验证：验证SQL语句的语义正确性
- AST生成：生成结构化的抽象语法树供后续模块使用

### 1.2 设计目标
- 支持标准SQL语法的解析
- 提供详细的语法错误信息
- 生成结构化的AST，便于后续处理
- 具有良好的可扩展性，支持新的SQL语法特性

## 2. 核心功能

### 2.1 词法分析
- 识别SQL关键字（SELECT、FROM、WHERE等）
- 识别标识符（表名、列名等）
- 识别字面量（字符串、数字等）
- 识别运算符和标点符号

### 2.2 语法分析
- 解析SELECT语句
- 解析INSERT语句
- 解析UPDATE语句
- 解析DELETE语句
- 解析CREATE TABLE语句

### 2.3 语义验证
- 验证表和列的存在性
- 验证表达式的类型兼容性
- 验证SQL语句的语义正确性

### 2.4 AST生成
- 构建结构化的抽象语法树
- 支持AST的遍历和操作
- 提供AST的序列化和反序列化功能

## 3. 类与接口设计

### 3.1 接口定义

#### Lexer接口
```rust
pub trait Lexer {
    fn tokenize(&self, sql: &str) -> Result<Vec<Token>, LexError>;
}
```

#### Parser接口
```rust
pub trait Parser {
    fn parse(&self, tokens: &[Token]) -> Result<AST, ParseError>;
    fn validate(&self, ast: &AST) -> Result<(), ValidationError>;
}
```

### 3.2 类定义

#### SqlLexer类
```rust
pub struct SqlLexer {
    keywords: HashSet<String>,
    symbols: HashSet<char>,
}

impl Lexer for SqlLexer {
    fn tokenize(&self, sql: &str) -> Result<Vec<Token>, LexError> {
        // 词法分析实现
    }
}
```

#### SqlParser类
```rust
pub struct SqlParser {
    lexer: Box<dyn Lexer>,
}

impl Parser for SqlParser {
    fn parse(&self, tokens: &[Token]) -> Result<AST, ParseError> {
        // 语法分析实现
    }
    
    fn validate(&self, ast: &AST) -> Result<(), ValidationError> {
        // 语义验证实现
    }
}
```

#### Token类
```rust
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub position: Position,
}
```

#### AST类
```rust
#[derive(Debug, PartialEq, Clone)]
pub struct AST {
    pub root: Box<dyn Node>,
    pub statements: Vec<Box<dyn Node>>,
}

impl AST {
    pub fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), Error> {
        // 访问者模式实现
    }
}
```

### 3.3 数据结构

#### TokenType枚举
```rust
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Keyword,
    Identifier,
    StringLiteral,
    NumericLiteral,
    Operator,
    Punctuation,
    Whitespace,
    Comment,
}
```

#### Position结构体
```rust
#[derive(Debug, PartialEq, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}
```

#### Statement枚举
```rust
#[derive(Debug, PartialEq, Clone)]
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
```

#### Expr枚举
```rust
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Column(String),
    Value(Value),
    Equal(Box<Expr>, Box<Expr>),
    LessThan(Box<Expr>, Box<Expr>),
    GreaterThan(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
}
```

#### Value枚举
```rust
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(i64),
    String(String),
    Null,
}
```

## 4. 执行流程

### 4.1 词法分析流程
1. 接收SQL语句字符串
2. 按字符遍历SQL语句
3. 根据当前字符类型识别Token
4. 生成Token流
5. 返回Token流或错误

### 4.2 语法分析流程
1. 接收Token流
2. 按语法规则解析Token流
3. 构建抽象语法树
4. 返回AST或语法错误

### 4.3 语义验证流程
1. 接收AST
2. 验证表和列的存在性
3. 验证表达式的类型兼容性
4. 验证SQL语句的语义正确性
5. 返回验证结果或错误

## 5. 异常处理

### 5.1 词法错误
- TokenizationError：词法分析过程中的错误
- InvalidCharacterError：遇到无效字符
- UnexpectedEOFError：意外的文件结束

### 5.2 语法错误
- ParseError：语法分析过程中的错误
- UnexpectedTokenError：遇到意外的Token
- MissingTokenError：缺少必要的Token
- InvalidSyntaxError：语法无效

### 5.3 语义错误
- ValidationError：语义验证过程中的错误
- TableNotFoundError：表不存在
- ColumnNotFoundError：列不存在
- TypeMismatchError：类型不匹配
- InvalidExpressionError：表达式无效

## 6. 性能考虑

### 6.1 词法分析优化
- 使用有限状态机提高Token识别速度
- 预编译关键字和符号集合，提高查找速度
- 批量处理字符，减少循环次数

### 6.2 语法分析优化
- 使用递归下降解析器，减少回溯
- 预计算语法规则，提高解析速度
- 缓存常用SQL语句的解析结果

### 6.3 内存优化
- 避免不必要的字符串复制
- 使用池化技术管理Token和AST节点
- 及时释放不再使用的内存

### 6.4 错误处理优化
- 提供详细的错误信息，包括错误位置和原因
- 实现错误恢复机制，允许部分解析
- 优化错误消息的生成和格式化

## 7. 测试策略

### 7.1 单元测试
- 测试词法分析器的Token识别
- 测试语法分析器的语句解析
- 测试语义验证器的验证功能
- 测试AST的构建和遍历

### 7.2 集成测试
- 测试完整的解析流程
- 测试各种SQL语句的解析
- 测试错误处理和恢复
- 测试边界情况和特殊输入

### 7.3 性能测试
- 测试解析大型SQL语句的性能
- 测试解析复杂SQL语句的性能
- 测试解析大量SQL语句的性能
- 测试内存使用情况

## 8. 扩展与维护

### 8.1 扩展SQL语法
- 定义新的Token类型
- 添加新的语法规则
- 扩展AST结构
- 更新语义验证逻辑

### 8.2 维护建议
- 定期更新SQL语法规则
- 优化解析器性能
- 改进错误处理和错误消息
- 添加新的测试用例

## 9. 总结

Parser模块是SQLRustGo的重要组成部分，负责将用户输入的SQL语句转换为结构化的抽象语法树。通过合理的设计和实现，Parser模块能够高效地解析和处理各种SQL语句，为后续的查询优化和执行提供基础。

未来的改进方向包括：
- 支持更多SQL语法特性
- 提高解析性能
- 增强错误处理能力
- 提供更友好的错误消息
- 支持SQL语句的格式化和美化