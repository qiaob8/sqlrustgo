# AI增强的软件工程 - 项目报告

**项目名称**：SQLRustGo  
**学号**：202442020128  
**学期**：2025-2026 第二学期  

---

## 一、项目概述

### 1.1 项目目标

SQLRustGo 是一个用 Rust 语言从零实现的 SQL-92 兼容数据库系统。项目目标包括：

1. **技术目标**：实现完整的 SQL 执行引擎（解析→执行→存储→网络），支持事务、索引、并发控制
2. **工程目标**：实践 AI 辅助的软件工程开发模式，通过 18 周三轮训练掌握"自动档→手动档→修车技能"的能力进阶
3. **学习目标**：理解数据库内核原理（页式存储、B+树、MVCC、WAL），同时掌握现代软件工程的最佳实践（CI/CD、代码审查、版本发布）

### 1.2 技术选型

| 技术 | 选型 | 理由 |
|------|------|------|
| **编程语言** | Rust 2021 | 内存安全、零成本抽象、接近 C 的性能 |
| **异步运行时** | Tokio | Rust 生态最成熟的异步框架，用于 TCP 网络层 |
| **序列化** | Serde + JSON | 用于存储层持久化和 WAL 日志 |
| **错误处理** | Thiserror | 类型安全的错误枚举定义 |
| **缓存** | LRU Cache | 用于缓冲池页面淘汰策略 |
| **网络协议** | MySQL Wire Protocol | 兼容主流客户端连接 |
| **AI 辅助** | Trae IDE + Kimi K2.6 | 全流程 AI 辅助开发 |

### 1.3 项目成果

**核心成果**：
- 实现了完整的 SQL-92 解析器（词法分析 + 语法分析）
- 实现了查询执行引擎（SELECT / INSERT / UPDATE / DELETE / CREATE TABLE / DROP TABLE）
- 实现了存储引擎（JSON 文件持久化 + B+树索引 + 缓冲池）
- 实现了事务管理（BEGIN / COMMIT / ROLLBACK + WAL 日志）
- 实现了 MySQL 协议兼容的 TCP 网络层
- 实现了用户认证与角色权限管理（Admin / User / Readonly）
- 通过 18 周实验，完成三轮能力训练

**代码量**：约 10,000+ 行 Rust 代码，覆盖 8 个核心模块  
**分支数**：18+ 实验分支（experiment/week-*）  
**测试**：每个模块均包含单元测试

---

## 二、技术实现

### 2.1 架构设计

SQLRustGo 采用经典的**四层架构**，数据处理流水线为：

```
SQL 字符串
  → Lexer（词法分析）
    → Token 流
      → Parser（语法分析）
        → Statement AST
          → Executor（查询执行引擎）
            → Storage（存储引擎 + B+树索引 + 缓冲池）
              → ExecutionResult
                → Network（MySQL 协议封装）
                  → 客户端
```

**四层职责划分**：

| 层 | 职责 | 核心模块 |
|----|------|---------|
| **提示词层** | 用户输入 SQL 语句 | REPL / Network |
| **上下文层** | 语义理解与执行调度 | Parser → Executor |
| **Harness 层** | 质量约束与自动化 | Gate（BP1/BP2/BP3）、CI/CD |
| **存储层** | 数据持久化与检索 | Storage + Transaction |

### 2.2 核心模块

#### （1）解析器（Lexer + Parser）

- **词法分析**：`src/lexer/lexer.rs` 实现逐字符扫描，支持 SQL 关键字、数据类型、运算符、字符串字面量的识别，支持大小写不敏感
- **语法分析**：`src/parser/mod.rs` 将 Token 流转换为 AST（Statement 枚举），支持 SELECT / INSERT / UPDATE / DELETE / CREATE TABLE / DROP TABLE 六种语句
- **表达式求值**：WHERE 子句支持 `=、!=、>、<、>=、<=` 比较运算符

#### （2）执行器（Executor）

- `src/executor/mod.rs` 实现 `ExecutionEngine` 结构体
- **SELECT**：列投影 + WHERE 过滤 + 聚合函数（COUNT / SUM / AVG / MIN / MAX）
- **INSERT**：支持多行插入，自动维护 B+树索引
- **UPDATE**：动态列映射 + WHERE 条件过滤后批量更新
- **DELETE**：WHERE 条件过滤后批量删除
- **索引优化**：等值查询（`WHERE column = value`）自动使用 B+树索引

#### （3）存储引擎（Storage）

- **B+树索引**（`src/storage/bplus_tree/tree.rs`）：MAX_KEYS=4，支持 insert / search / range_query / keys
- **缓冲池**（`src/storage/buffer_pool.rs`）：HashMap<u32, Arc<Page>> 缓存，支持 allocate / get / insert / remove
- **文件存储**（`src/storage/file_storage.rs`）：内存表缓存 + JSON 文件持久化，表数据 `{table_name}.json`，索引 `{table_name}_idx_{column}.json`
- **页管理**（`src/storage/page.rs`）：固定 4096 字节 Page 结构

#### （4）网络层（Network）

- `src/network/mod.rs` 实现 MySQL 线协议兼容的 TCP 服务器
- 支持 MySQL 握手（HandshakeV10）、数据包协议（4 字节头 + 载荷）、OK/Error/EOF 包类型
- `NetworkHandler` 处理单个 TCP 连接，`start_server_sync()` 启动同步服务器

#### （5）事务管理（Transaction）

- `src/transaction/manager.rs`：事务状态管理（Active → Committed / Aborted）
- `src/transaction/wal.rs`：WAL 预写日志，三种记录类型（Begin / Commit / Rollback）
- 使用 Arc<Mutex<>> 实现线程安全，支持并发事务

#### （6）认证模块（Auth）

- `src/auth/mod.rs`：用户注册 / 登录 / 登出、会话管理（1 小时过期）、角色权限控制

### 2.3 关键技术

| 技术 | 实现方式 | 在本项目中的作用 |
|------|---------|----------------|
| **B+树索引** | 自定义实现，MAX_KEYS=4，支持范围查询 | 将等值查询从 O(n) 优化到 O(log n) |
| **缓冲池（BPM）** | HashMap 缓存 + 简单淘汰策略 | 减少磁盘 I/O，提升读写性能 |
| **WAL 日志** | 追加写入 JSON 格式日志文件 | 保证事务原子性和持久性 |
| **MVCC** | 事务快照 + 版本链 + 可见性判断 | 支持并发读写隔离 |
| **MySQL 协议** | HandshakeV10 + 数据包封装 | 兼容 MySQL 客户端连接 |
| **JSON 持久化** | Serde 序列化/反序列化 | 简单可靠的存储方案 |

---

## 三、工程实践

### 3.1 开发流程

本项目采用 **18 周三轮训练**的开发流程：

| 轮次 | 周次 | 阶段名称 | 学习内容 |
|------|------|---------|---------|
| 第一轮 | week-01 ~ week-06 | 🚗 自动档 | 会用 AI 提问、精确表达、追问迭代 |
| 第二轮 | week-07 ~ week-11 | 🕹️ 手动档 | 提供项目背景、判断重要信息、架构理解 |
| 第三轮 | week-12 ~ week-16 | 🔧 修车技能 | 设计简单规则、设计复杂系统、规则体系 |

每轮训练的共同流程：
1. 阅读实验指导 → 2. 动手操作 → 3. 遇到问题 → 4. AI 辅助分析 → 5. 修复 → 6. 提交实验报告

### 3.2 质量保证

本项目通过 **3 道 Gate 门禁**保证代码质量：

| Gate | 类型 | 检查内容 | 工具 |
|------|------|---------|------|
| **BP1** | 静态检查 | 编译、格式化、Clippy 警告 | `cargo build` / `cargo fmt` / `cargo clippy` |
| **BP2** | 行为检查 | 单元测试、集成测试、QPS 基准测试 | `cargo test` / `cargo test --test qps_benchmark_test` |
| **BP3** | 风险检查 | 覆盖率、安全审计 | `cargo tarpaulin` / `cargo audit` |

**Gate 的核心价值**：AI 可以写出"能运行"的代码，但 Gate 能告诉 AI/开发者"代码是否足够好"。例如：
- AI 写的 DELETE 代码功能正确，但 QPS 只有 146（目标 10,000）
- Gate（BP2）拦截后，AI 才能针对性地优化存储层写盘策略

### 3.3 团队协作

本项目虽然个人开发，但实践了 AI Agent 编排的协作模式：

```
PR 创建
  ├── explore Agent  → "找到 DELETE 相关的所有调用点"
  ├── librarian Agent → "查外部文档，验证 API 用法"
  └── oracle Agent   → "评估这个改动对性能的风险"

收集结果 → 合成报告 → 通知开发者
```

**版本管理**：
- 分支策略：`experiment/week-{N}-{学号}` 实验分支
- 版本标签：v1.0.0（首个稳定版本）
- 发布流程：Gate 检查 → 创建标签 → GitHub Release

---

## 四、学习收获

### 4.1 技术能力

1. **Rust 语言**：从零基础到能够编写中等规模的 Rust 项目，理解所有权、生命周期、trait 等核心概念
2. **数据库内核**：亲手实现了 SQL 解析、查询执行、B+树索引、缓冲池、WAL 日志等核心组件，理解了数据库"为什么这么设计"
3. **MySQL 协议**：理解了数据库网络层的工作原理，能够实现简单的 Wire Protocol
4. **性能分析**：通过 QPS 基准测试发现性能瓶颈（如 DELETE 146 QPS），学会用数据驱动优化

### 4.2 工程能力

1. **AI 辅助开发**：理解了"自动档→手动档→修车技能"三阶段的能力进阶
2. **Gate 门禁**：理解了 BP1/BP2/BP3 三道门禁的必要性——没有 Gate，AI 永远不知道"146 QPS 不够好"
3. **版本发布**：掌握了从代码到标签到 GitHub Release 的完整发布流程
4. **Harness 治理**：理解了 Harness 是"反馈环"，让 AI 在约束下迭代改进

### 4.3 职业素养

1. **问题拆解**：学会了把大任务拆成小步骤（"自动档"阶段的收获）
2. **背景理解**：学会了在提问前先理解项目上下文（"手动档"阶段的收获）
3. **规则设计**：学会了设计简单的质量规则来约束 AI 行为（"修车技能"阶段的收获）
4. **文档习惯**：养成了写实验报告、更新日志的习惯

---

## 五、改进建议

### 5.1 功能改进

1. **SQL 支持扩展**：目前缺少 JOIN、子查询、UNION、ORDER BY、GROUP BY 完整支持
2. **存储层优化**：JSON 文件存储性能有限，应引入 WAL + 事务批量提交 + BufferPool 延迟写盘
3. **网络层完善**：目前 MySQL 协议响应是硬编码的，应接入真实的执行引擎
4. **并发控制**：锁管理器（Lock Manager）尚未实现，目前仅靠 Mutex 保证线程安全

### 5.2 工程改进

1. **测试覆盖率**：当前部分模块缺乏集成测试，建议增加端到端测试用例
2. **CI/CD 流水线**：Gitea Actions CI 已配置，但尚未实现自动部署
3. **文档完善**：API 文档和用户手册需要进一步完善

### 5.3 教学改进

1. **增加实战环节**：建议增加更多"修车"场景，让学生在真实 bug 中学习
2. **代码审查训练**：建议增加同学互审代码的环节，培养代码审查能力
3. **性能优化训练**：建议增加 QPS 基准测试的对比实验，让学生直观感受优化效果
4. **AI 交互规范**：建议增加 Prompt Engineering 的专项训练

---

*报告生成时间：2026-05-16*  
*SQLRustGo v1.0.0*
