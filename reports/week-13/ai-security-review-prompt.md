# AI 安全审查 Prompt 模板

## 审查对象

文件：`src/storage/file_storage.rs`
函数：`build_index`
行号：第 256-265 行

## 实际代码

```rust
// Build B+ Tree from existing rows
let mut index = BPlusTree::new();
for (row_id, row) in table.rows.iter().enumerate() {
    if let Some(value) = row.get(column_index) {
        if let Value::Integer(key) = value {
            index.insert(*key, row_id as u32);
        }
    }
}
```

## 使用的 Prompt

```
审查 src/storage/file_storage.rs 第 256-265 行的索引构建代码：
for (row_id, row) in table.rows.iter().enumerate() {
    if let Some(value) = row.get(column_index) {
        if let Value::Integer(key) = value {
            index.insert(*key, row_id as u32);
        }
    }
}
请检查：
1. SQL 注入风险
2. 缓冲区溢出风险
3. 敏感信息泄露
4. 不安全的加密使用
5. 其他安全问题（如整数截断、整数溢出、负数处理等）
```

## AI 审查结果

| 风险项            | 严重程度 | 描述               | 修复建议                    |
| -------------- | ---- | ---------------- | ----------------------- |
| `as u32` 截断    | **中** | row_id > 2^32 静默截断 | `u32::try_from(row_id)` |
| `i64 as u32` 截断 | **中** | 负数 i64 截断为 u32    | 校验 `key >= 0`          |
| 整数溢出 row_id   | 低    | 单进程 row_id 不会超过 2^63 | 长期运行需监控               |
| 竞态 index.insert | 低    | 单线程 build 阶段安全   | rebuild 阶段需加锁           |
| SQL 注入         | 0    | 不涉及用户输入         | —                       |
| 缓冲区溢出         | 0    | Rust 借用检查器保证    | —                       |
| 敏感信息泄露        | 0    | 不涉及密钥           | —                       |

## 关键代码位置

```rust
// src/storage/file_storage.rs:256-265
for (row_id, row) in table.rows.iter().enumerate() {  // line 256-257
    if let Some(value) = row.get(column_index) {        // line 258
        if let Value::Integer(key) = value {            // line 259
            index.insert(*key, row_id as u32);          // line 260 ★ 风险点
        }                                                // line 261
    }                                                    // line 262
}                                                        // line 263
```

## 修复建议

```rust
// 修复版：使用 try_from
use std::convert::TryFrom;
let row_id_u32 = u32::try_from(row_id)
    .map_err(|_| std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "row_id exceeds u32 range"
    ))?;
index.insert(*key, row_id_u32);
```

---

*使用工具：Claude Code（Cursor）*
*审查日期：2026-06-14*
