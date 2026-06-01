# 实验报告

| 项目       | 内容              |
| -------- | --------------- |
| **实验名称** | AI辅助核心模块实现 |
| **实验周次** | 第 7 周           |
| **实验日期** | 2026 年 4 月 25 日  |
| **学生姓名** | 阳奇              |
| **学号**   | 202442020128    |
| **班级**   | 2024级软件工程1班     |
| **指导教师** | 李莹              |

---

## 一、实验目的

1. 掌握AI辅助开发的方法和流程
2. 使用AI辅助实现词法分析器
3. 使用AI辅助实现语法分析器
4. 使用AI辅助实现存储引擎（页结构和缓冲池）
5. 编写测试用例验证实现正确性
6. 理解AI辅助开发的优势和局限性

---

## 二、实验环境

### 2.1 硬件环境

| 项目    | 配置                                     |
| ----- | -------------------------------------- |
| 计算机型号 | LENOVO 83DG                            |
| CPU   | Intel(R) Core(TM) i7-14650HX (16核24线程) |
| 内存    | 16GB                                   |
| 硬盘    | C盘: 300GB, D盘: 650GB                   |

### 2.2 软件环境

| 软件    | 版本                                    |
| ----- | ------------------------------------- |
| 操作系统  | Microsoft Windows 11 专业版 (10.0.26200) |
| Rust  | 1.95.0                                 |
| Git   | 2.45.2.windows.1                      |
| IDE   | Trae IDE                              |
| AI工具 | GitHub Copilot, Claude 3.5 Sonnet      |

---

## 三、实验内容与步骤

### 3.1 AI辅助开发概述

#### AI辅助开发流程

1. **明确需求**：清楚描述要实现的功能
2. **设计提示词**：编写清晰、完整的提示词
3. **生成代码**：使用AI生成初始代码
4. **代码审查**：人工审查AI生成的代码
5. **测试验证**：编写测试用例，验证功能
6. **迭代优化**：根据反馈优化代码

#### 提示词工程原则

- **清晰性**：明确任务目标，避免歧义
- **完整性**：提供必要上下文，指定约束条件
- **结构性**：使用结构化格式，分步骤描述
- **可迭代性**：便于反馈修正，支持渐进优化

---

### 3.2 AI辅助实现词法分析器

#### 步骤1：设计Token定义

**提示词设计**：
```
设计一个SQL词法分析器的Token枚举，支持：
- SQL关键字：SELECT, FROM, WHERE, INSERT, UPDATE, DELETE, CREATE, DROP, TABLE
- 标识符：表名、列名
- 字面量：字符串、整数、浮点数、布尔值
- 运算符：=, <>, <, >, <=, >=, +, -, *, /, AND, OR, NOT
- 分隔符：, ( ) ; .
使用Rust枚举实现，包含Debug和Clone trait。
```

**AI输出**：参考 `src/lexer/token.rs`

#### 步骤2：实现词法分析器

**提示词设计**：
```
基于以下Token定义，实现一个SQL词法分析器：
[Token定义代码]
要求：
1. 实现Lexer结构体，包含input和position字段
2. 实现next_token()方法，返回下一个Token
3. 支持跳过空白字符（空格、制表符、换行）
4. 支持识别关键字和标识符（区分大小写）
5. 支持识别整数和浮点数字面量
6. 支持识别字符串字面量（单引号）
7. 支持识别运算符和分隔符
8. 使用Rust实现，考虑错误处理
```

**代码审查要点**：
- 检查逻辑是否正确
- 检查错误处理是否完善
- 检查边界条件是否处理

**实现文件**：`src/lexer/lexer.rs`

---

### 3.3 AI辅助实现语法分析器

#### 步骤1：设计AST定义

**提示词设计**：
```
设计SQL语句的AST节点，支持：
- SELECT语句：columns（列列表）, table（表名）, where_clause（WHERE条件）
- INSERT语句：table（表名）, columns（列列表）, values（值列表）
- UPDATE语句：table（表名）, set_clauses（SET子句）, where_clause（WHERE条件）
- DELETE语句：table（表名）, where_clause（WHERE条件）
- CREATE TABLE语句：name（表名）, columns（列定义列表）
- DROP TABLE语句：name（表名）
使用Rust结构体和枚举实现，包含Debug和Clone trait。
```

#### 步骤2：实现语法分析器

**提示词设计**：
```
基于以下Token和AST定义，实现一个SQL语法分析器：
[Token定义代码]
[AST定义代码]
要求：
1. 实现Parser结构体，包含tokens和position字段
2. 实现parse_statement()方法，解析SQL语句并返回AST
3. 支持解析SELECT、INSERT、UPDATE、DELETE、CREATE TABLE、DROP TABLE语句
4. 使用Rust实现，考虑错误处理
5. 提供清晰的错误信息
```

**代码审查要点**：
- 检查解析逻辑是否正确
- 检查错误处理是否完善
- 检查是否支持所有SQL语句

**实现文件**：`src/parser/mod.rs`

---

### 3.4 AI辅助实现存储引擎

#### 步骤1：设计页结构

**提示词设计**：
```
设计数据库存储页结构，要求：
1. 页大小：4KB
2. 页头：页ID（4字节）
3. 数据区：存储实际数据
4. 方法：page_id()获取页面ID
5. 使用Rust实现，考虑内存安全
6. 支持克隆操作
```

**实现文件**：`src/storage/page.rs`

#### 步骤2：实现缓冲池

**提示词设计**：
```
实现数据库缓冲池管理器，要求：
1. 容量可配置（默认10页）
2. 使用LRU置换算法思想
3. 支持get(page_id)获取页面
4. 支持insert(page)插入页面
5. 支持allocate(page_id)分配新页面
6. 支持remove(page_id)移除页面
7. 使用Rust实现，考虑线程安全
8. 使用HashMap存储页面
```

**代码审查要点**：
- 检查并发安全是否保证
- 检查页面管理是否完善
- 检查LRU思想是否正确实现

**实现文件**：`src/storage/buffer_pool.rs`

---

### 3.5 测试验证

#### 测试用例设计

**词法分析器测试**：
- 测试关键字识别（SELECT, FROM, WHERE等）
- 测试标识符识别（表名、列名）
- 测试字面量识别（字符串、数字、布尔值）
- 测试运算符和分隔符识别
- 测试大小写不敏感

**语法分析器测试**：
- 测试SELECT语句解析
- 测试INSERT语句解析（单列、多列、多行）
- 测试UPDATE语句解析
- 测试DELETE语句解析
- 测试CREATE TABLE语句解析
- 测试DROP TABLE语句解析
- 测试聚合函数解析（COUNT, SUM, AVG, MIN, MAX）

**存储引擎测试**：
- 测试页结构创建和访问
- 测试缓冲池插入和获取
- 测试缓冲池分配和移除
- 测试缓冲池容量限制和淘汰

---

## 四、实验结果

### 4.1 测试执行结果

运行 `cargo test --lib` 验证所有测试通过：

```
running 97 tests
...
test result: ok. 97 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.12s
```

### 4.2 完成情况

| 任务       | 状态   | 说明                     |
| -------- | ---- | ---------------------- |
| 词法分析器实现 | ✅完成  | 实现了Token枚举和Lexer结构体，支持SQL关键字、标识符、字面量、运算符、分隔符 |
| 语法分析器实现 | ✅完成  | 实现了AST定义和Parser结构体，支持SELECT、INSERT、UPDATE、DELETE、CREATE/DROP TABLE |
| 页结构实现 | ✅完成  | 实现了Page结构体，支持4KB固定大小页面 |
| 缓冲池实现 | ✅完成  | 实现了BufferPool结构体，支持页面缓存和淘汰 |
| 测试用例编写 | ✅完成  | 编写了97个测试用例，覆盖核心功能 |
| AI辅助开发实践 | ✅完成  | 使用AI生成初始代码，人工审查优化 |

### 4.3 生成的文件

| 文件路径 | 描述 |
| -------- | ---- |
| `src/lexer/token.rs` | Token枚举定义 |
| `src/lexer/lexer.rs` | 词法分析器实现 |
| `src/parser/mod.rs` | 语法分析器和AST定义 |
| `src/storage/page.rs` | 页结构实现 |
| `src/storage/buffer_pool.rs` | 缓冲池实现 |

---

## 五、实验心得与总结

### 5.1 AI辅助开发的优势

1. **提高效率**：AI可以快速生成样板代码，减少重复性工作
2. **降低门槛**：新手可以借助AI快速上手复杂功能
3. **提供参考**：AI可以提供最佳实践建议和代码示例
4. **加速学习**：AI可以解释概念，帮助理解技术原理

### 5.2 AI辅助开发的局限性

1. **上下文窗口限制**：AI无法理解整个大型项目的上下文
2. **创造性限制**：AI难以创造全新的解决方案
3. **领域知识限制**：AI缺乏特定领域的专业知识
4. **责任问题**：AI生成的代码需要人工负责审查

### 5.3 代码审查要点

在使用AI生成代码后，需要重点审查：
- **正确性**：代码是否实现了预期功能
- **安全性**：是否存在安全漏洞
- **性能**：是否存在性能问题
- **可读性**：代码是否易于理解
- **可维护性**：代码是否易于修改

---

## 六、思考题

### 6.1 AI辅助开发如何提高开发效率？

**答案**：AI辅助开发通过以下方式提高效率：
1. **自动生成代码**：根据需求描述自动生成代码框架和实现
2. **代码补全**：智能补全代码片段，减少输入量
3. **文档生成**：自动生成注释和文档
4. **测试用例生成**：根据代码自动生成测试用例
5. **重构建议**：提供代码优化和重构建议

### 6.2 如何确保AI生成代码的质量？

**答案**：确保AI生成代码质量需要：
1. **严格的代码审查**：人工审查每一行AI生成的代码
2. **测试驱动开发**：编写测试用例验证功能正确性
3. **代码规范检查**：使用lint工具检查代码规范
4. **性能测试**：验证代码的性能表现
5. **安全审计**：检查潜在的安全漏洞

---

## 七、附录

### 7.1 核心代码示例

**Token枚举（src/lexer/token.rs）**：
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // 关键字
    Select, From, Where, Insert, Update, Delete,
    Create, Drop, Table,
    
    // 标识符和字面量
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(String),
    BooleanLiteral(bool),
    
    // 运算符
    Equal, NotEqual, Greater, Less, GreaterEqual, LessEqual,
    Plus, Minus, Star, Slash,
    And, Or, Not,
    
    // 分隔符
    Comma, LParen, RParen, Semicolon, Dot,
    
    // 特殊
    Eof,
}
```

**Lexer结构（src/lexer/lexer.rs）**：
```rust
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, position: 0 }
    }
    
    pub fn next_token(&mut self) -> Token {
        // 词法分析逻辑
    }
}
```

**Parser结构（src/parser/mod.rs）**：
```rust
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn parse_statement(&mut self) -> Result<Statement, String> {
        // 语法分析逻辑
    }
}
```

**Page结构（src/storage/page.rs）**：
```rust
pub struct Page {
    pub page_id: u32,
    pub data: Vec<u8>,
}

impl Page {
    pub fn new(page_id: u32) -> Self {
        Self {
            page_id,
            data: vec![0u8; 4096],
        }
    }
}
```

**BufferPool结构（src/storage/buffer_pool.rs）**：
```rust
pub struct BufferPool {
    pages: Mutex<HashMap<u32, Arc<Page>>>,
    capacity: usize,
}

impl BufferPool {
    pub fn get(&self, page_id: u32) -> Option<Arc<Page>> {
        // 获取页面逻辑
    }
    
    pub fn insert(&self, page: Arc<Page>) {
        // 插入页面逻辑（含淘汰机制）
    }
}
```

### 7.2 测试脚本

**测试示例**：
```rust
#[test]
fn test_lexer_select() {
    let mut lexer = Lexer::new("SELECT id FROM users");
    assert_eq!(lexer.next_token(), Token::Select);
    assert_eq!(lexer.next_token(), Token::Identifier("id".to_string()));
    assert_eq!(lexer.next_token(), Token::From);
    assert_eq!(lexer.next_token(), Token::Identifier("users".to_string()));
}

#[test]
fn test_parser_select() {
    let result = parse("SELECT id, name FROM users WHERE age > 18");
    assert!(result.is_ok());
    match result.unwrap() {
        Statement::Select(s) => {
            assert_eq!(s.table, "users");
            assert_eq!(s.columns.len(), 2);
            assert!(s.where_clause.is_some());
        }
        _ => panic!("Expected SELECT statement"),
    }
}

#[test]
fn test_buffer_pool() {
    let pool = BufferPool::new(10);
    let page = Arc::new(Page::new(1));
    pool.insert(page);
    
    let retrieved = pool.get(1);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().page_id(), 1);
}
```

---

| 指导教师 | __________________ | 实验成绩   | __________________ |
| ---- | ------------------------ | ------ | ------------------------ |
| 批改日期 | __________________ | <br /> | <br />                   |