# 安全审计报告

> **生成日期**：2026-06-20
> **审计工具**：`cargo audit v0.22.2` + `cargo clippy` + 静态模式统计 + AI 安全审查
> **审计范围**：SQLRustGo 全项目
> **数据来源**：[`reports/week-13/cargo-audit-result.json`](file:///d:/sqlrustgo/project-main/reports/week-13/cargo-audit-result.json)（真实 `cargo audit --json` 输出）

---

## 1. 依赖安全

| 依赖           | 版本      | 漏洞数 | 状态         |
| ------------ | ------- | --- | ---------- |
| serde        | 1.0.x   | 0   | ✅ 安全      |
| serde_json   | 1.0.x   | 0   | ✅ 安全      |
| tokio        | 1.41.0  | 0   | ✅ 安全      |
| bincode      | 1.3.x   | 0   | ✅ 安全      |
| dashmap      | 5.x     | 0   | ✅ 安全      |
| regex        | 1.x     | 0   | ✅ 安全      |
| chrono       | 0.4.x   | 0   | ✅ 安全      |
| thiserror    | 1.x     | 0   | ✅ 安全      |
| anyhow       | 1.x     | 0   | ✅ 安全      |
| uuid         | 1.x     | 0   | ✅ 安全      |
| criterion    | 0.5.x   | 0   | ✅ 安全      |
| **合计**       | 152 个依赖 | 0   | **✅ 零漏洞** |

**扫描源**：

```bash
cargo audit --json
# → vulnerabilities.found: false, count: 0
# → database: 1134 advisories scanned
# → lockfile: 152 crate dependencies
```

---

## 2. 代码安全

| 检查项     | 结果     | 风险等级 | 状态       |
| ------ | ------ | ---- | -------- |
| unwrap() 使用 | 630 处 | 中    | ⚠️ 待优化  |
| expect() 使用 | 30 处  | 中    | ⚠️ 待优化  |
| unsafe 代码 | **0 处** | —    | ✅ **零 unsafe** |
| TODO    | 1 处    | 低    | 计划中      |
| clippy 错误 | 0     | —    | ✅       |
| clippy 警告 | 2（风格） | —    | 非安全问题    |

**clippy 警告**（2 条风格警告，非安全）：

```
warning: this `if let` can be collapsed into the outer `if let`
  --> src\executor\mod.rs:389:25

warning: this `if let` can be collapsed into the outer `if let`
  --> src\storage\file_storage.rs:260:17

warning: `sqlrustgo` (lib) generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.66s
```

> 完整日志：[`reports/week-13/clippy-output.log`](file:///d:/sqlrustgo/project-main/reports/week-13/clippy-output.log)

---

## 3. 建议修复项

### 3.1 高优先级

1. **修复 `as u32` 截断风险**：`src/storage/file_storage.rs` 第 263 行 `row_id as u32`，当 `row_id > 2^32` 时静默截断；改用 `u32::try_from(row_id).expect("row_id overflow")`
2. **审查 unsafe 代码**：当前 0 处，建议加 `#![deny(unsafe_code)]` 防止未来误用

### 3.2 中优先级

3. **减少 unwrap() 使用**：执行器 [`src/executor/mod.rs`](file:///d:/sqlrustgo/project-main/src/executor/mod.rs) 416 个 unwrap（占 66%），改为 `?` 传播错误
4. **网络层 Result 化**：[`src/network/mod.rs`](file:///d:/sqlrustgo/project-main/src/network/mod.rs) 47 个 unwrap 失败 = 服务器进程崩溃；改为 graceful error
5. **WAL 关键路径 Result 化**：[`src/transaction/wal.rs`](file:///d:/sqlrustgo/project-main/src/transaction/wal.rs) 37 个 unwrap，WAL 写入失败不可静默吞掉

### 3.3 低优先级

6. **CI 集成 cargo audit**：把 `cargo audit` 加入 BP1（参考 [`.gitea/workflows/ci.yml`](file:///d:/sqlrustgo/project-main/.gitea/workflows/ci.yml)）
7. **增加 secret scanning**：用 `gitleaks` 自动扫描密钥泄露
8. **完善 TODO 跟踪**：1 处 TODO 转为 issue 跟踪
9. **增加 PreparedStatement API**：从源头杜绝 SQL 注入

---

*最后更新: 2026-06-20*
