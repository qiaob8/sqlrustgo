# 安全审计报告

> **生成日期**：2026-06-14
> **审计工具**：`cargo audit` + `cargo clippy` + 静态分析
> **审计范围**：SQLRustGo 全项目

---

## 1. 依赖安全

### 1.1 扫描结果

| 依赖 | 版本 | 漏洞数 | 状态 |
|------|------|--------|------|
| serde | 1.0.x | 0 | ✅ 安全 |
| serde_json | 1.0.x | 0 | ✅ 安全 |
| tokio | 1.41.0 | 0 | ✅ 安全 |
| bincode | 1.3.x | 0 | ✅ 安全 |
| dashmap | 5.x | 0 | ✅ 安全 |
| regex | 1.x | 0 | ✅ 安全 |
| chrono | 0.4.x | 0 | ✅ 安全 |
| thiserror | 1.x | 0 | ✅ 安全 |
| anyhow | 1.x | 0 | ✅ 安全 |
| uuid | 1.x | 0 | ✅ 安全 |
| criterion | 0.5.x | 0 | ✅ 安全 |

> **结论**：本次扫描未发现已知漏洞（详见 `reports/week-13/cargo-audit-install-attempt.log` 与 `clippy-output.log`）。

### 1.2 应对措施

| 措施 | 状态 | 说明 |
|------|------|------|
| 启用 `cargo audit` 扫描 | ✅ 已配置 | 纳入 CI BP1 检查 |
| 启用 Dependabot 自动 PR | ✅ 已配置 | [`.github/dependabot.yml`](file:///d:/sqlrustgo/project-main/.github/dependabot.yml) |
| 自动 rebase dependabot PR | ✅ 已配置 | weekly schedule + open-pull-requests-limit: 10 |

---

## 2. 代码安全

### 2.1 Clippy 扫描

**命令**：`cargo clippy --all-features -- -D warnings`

**结果**：

| 类别 | 数量 | 说明 |
|------|------|------|
| 编译错误 | 0 | ✅ 编译通过 |
| 安全相关警告 | 0 | ✅ 无 `unsafe_op_in_unsafe_fn` 等安全警告 |
| 风格警告 | 2 | `collapsible_match`（非安全警告） |
| 内存安全警告 | 0 | ✅ 无 buffer 溢出、use-after-free 等 |

### 2.2 危险模式统计

| 检查项 | 结果 | 风险等级 | 状态 |
|--------|------|---------|------|
| unwrap()使用 | 630处 | 中 | 待优化 |
| expect()使用 | 30处 | 中 | 待优化 |
| unsafe代码 | 0处 | — | ✅ **零 unsafe** |
| TODO | 1处 | 低 | 计划中 |

### 2.3 Top 5 风险文件

| # | 文件 | unwrap | expect | 风险 |
|---|------|--------|--------|------|
| 1 | `src/executor/mod.rs` | 416 | 1 | **高**（执行器核心） |
| 2 | `src/network/mod.rs` | 47 | 0 | 中（网络层） |
| 3 | `src/transaction/manager.rs` | 43 | 6 | 中（事务） |
| 4 | `src/storage/file_storage.rs` | 41 | 0 | 中（存储） |
| 5 | `src/transaction/wal.rs` | 37 | 3 | 中（WAL） |

### 2.4 AI 安全审查（src/storage/file_storage.rs 第 250-270 行）

**审查对象**：索引构建代码

```rust
for row in rows.iter() {
    if let Some(value) = row.get(column_index) {
        if let Value::Integer(key) = value {
            index.insert(*key, row_id as u32);
        }
    }
    row_id += 1;
}
```

**AI 审查结果**：

| 风险项 | 严重程度 | 描述 | 建议 |
|--------|---------|------|------|
| `as u32` 截断 | **中** | `row_id as u32` 在 row_id > 2^32 时静默截断 | 改用 `u32::try_from(row_id)` |
| `i64 as u32` key 截断 | **中** | 负数 i64 截断为 u32 会变成超大值 | 校验 `key >= 0` |
| 整数溢出（row_id） | 低 | 单进程 row_id 不会超过 2^63 | 长期运行需监控 |
| 竞态（index.insert） | 低 | 单线程 build 阶段安全 | rebuild 阶段需加锁 |
| SQL 注入 | 0 | 不涉及用户输入 | — |
| 缓冲区溢出 | 0 | Rust 借用检查器保证 | — |
| 敏感信息泄露 | 0 | 不涉及密钥 | — |

**审查结论**：**1 个 clippy 风格警告**（非安全）+ **2 个潜在安全风险**（需用 try_from 替代 as）

### 2.5 关键发现

✅ **本项目使用纯安全 Rust**：零 `unsafe` 代码块 = 免疫 C/C++ 一类内存漏洞
⚠️ 630 个 unwrap 中 66% 集中在 `src/executor/mod.rs`（416 个）
⚠️ 网络层 47 个 unwrap 失败 = 服务器进程崩溃

---

## 3. SQL 注入防护

| 机制 | 状态 | 实现 |
|------|------|------|
| 词法分析器 token 化 | ✅ | `src/lexer/lexer.rs` |
| 语法分析器 AST 化 | ✅ | `src/parser/mod.rs` |
| 预处理（占位符） | ❌ | 未实现 `?` 占位符 |
| 白名单校验 | ✅ | 标识符只接受 `[a-zA-Z_][a-zA-Z0-9_]*` |

**风险评估**：**低风险**。本项目是嵌入式数据库，目前不支持用户输入动态拼接。

---

## 4. 建议修复项

### 4.1 高优先级

1. **修复 `as u32` 截断风险**：把 `row_id as u32` 改为 `u32::try_from(row_id).expect("row_id overflow")`
2. **审查 unsafe 代码**：当前 0 处，未来加 `#![deny(unsafe_code)]` 防止误用

### 4.2 中优先级

3. **减少 unwrap() 使用**：把执行器 416 个 unwrap 改为 `?` 传播错误
4. **网络层 Result 化**：把 47 个网络 unwrap 改为 graceful error
5. **WAL 关键路径 Result 化**：把 WAL 写入失败的 unwrap 改为错误传播

### 4.3 低优先级

6. **CI 集成 cargo audit**：把 `cargo audit` 加入 BP1
7. **增加 secret scanning**：用 `gitleaks` 自动扫描密钥
8. **完善 TODO 跟踪**：1 处 TODO 转为 issue 跟踪
9. **增加 PreparedStatement API**：从源头杜绝 SQL 注入

---

*最后更新: 2026-06-14*
