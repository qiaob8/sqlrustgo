# SQLRustGo v1.0.0 发布说明

> 发布日期：2026 年 6 月 29 日
> 标签：v1.0.0
> 提交人：阳奇（202442020128）

---

## 发布概述

SQLRustGo v1.0.0 是首个**稳定版本**，标志着项目从开发阶段进入生产就绪状态。本版本完成了一个用 Rust 实现的小型 SQL 数据库引擎，覆盖了 SQL-92 子集的核心 DML/DDL 操作、页式存储、B+ 树索引、MVCC 事务、TCP/MySQL 协议兼容服务器等关键能力。

本学期围绕该版本，完成了 1-15 周的"软件工程+Harness 治理"完整闭环实验。

---

## 新功能

### SQL 支持

- 支持 `SELECT` 语句（含 WHERE、ORDER BY、LIMIT、LIKE、BETWEEN、IN、IS NULL 等子句）
- 支持 `INSERT` 语句（单行/多行）
- 支持 `UPDATE` 语句（含 SET ... WHERE 条件更新）
- 支持 `DELETE` 语句（含 WHERE 条件删除）
- 支持 `CREATE TABLE` 语句（含列定义、PRIMARY KEY 约束）
- 支持 `DROP TABLE` 语句
- 支持聚合函数（COUNT、SUM、AVG、MIN、MAX）
- 支持 DISTINCT 去重查询

### 存储引擎

- 页式存储（4KB/页）
- 缓冲池管理（BufferPool + LRU 淘汰）
- B+ 树索引
- WAL（预写日志）机制
- 表/索引 JSON 持久化

### 事务

- MVCC（多版本并发控制）
- 快照隔离级别
- 事务日志

### 网络

- TCP 服务器
- MySQL 协议兼容（基础子集）
- REPL 交互式终端

### Harness 治理（学期成果）

- Gate 检查流程（BP1 静态 + BP2 行为）
- QPS 基准测试体系（DELETE/UPDATE/INSERT/SELECT）
- 根因分析 + 优化闭环
- 三层模型（提示词-上下文-Harness）反思

---

## 质量指标

| 指标        | 数值                |
| --------- | ----------------- |
| 测试用例      | 4 passed, 0 failed |
| QPS 基准（SELECT） | 3097              |
| QPS 基准（DELETE） | 146               |
| QPS 基准（UPDATE） | 83                |
| QPS 基准（INSERT） | 88                |
| Clippy 警告  | 0（修复 2 个 collapsible_match）|
| 文档完整度     | 90%               |

---

## 破坏性变更

无

---

## 升级指南

无需特殊配置，直接替换 Binary 即可。

如果从 `v0.1.0-alpha` 升级：
- 旧的 JSON 数据文件结构兼容，无需迁移
- WAL 日志文件会自动 rebase

---

## 已知问题

- `cargo build --release` 在 Windows 上存在工具链 build-script 问题（仅限 `--release` profile；`cargo test` 不受影响）
- DELETE/UPDATE 性能仍待优化（QPS < 10,000），下一步将引入 WAL + 批量提交优化

---

## 致谢

感谢所有贡献者的辛勤付出！

特别致谢：

- 指导教师 李莹
- 实验伙伴与 Harness 治理实验小组成员
- Rust 社区与开源工具链

---

*详细实验过程见 `reports/` 目录下的各周实验报告。*
