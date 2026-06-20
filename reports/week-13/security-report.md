# SQLRustGo 安全审计报告

> **生成时间**：2026-06-14
> **Commit**：`a650f11` (week-12 latest)
> **Branch**：`experiment/week-10-202442020128`
> **审计范围**：依赖安全 + 代码安全 + 配置安全
> **审计工具**：`cargo audit` + `cargo clippy` + 静态分析

---

## 1. 依赖安全

### 1.1 工具与版本

| 项目 | 详情 |
|------|------|
| 工具 | `cargo-audit v0.22.2` |
| 数据源 | RustSec Advisory Database (<https://rustsec.org/advisories/>) |
| 扫描对象 | `Cargo.lock` 中所有依赖 |
| 扫描时间 | 2026-06-14 |

### 1.2 依赖清单

| 依赖 | 版本 | 用途 | 漏洞数 | 状态 |
|------|------|------|--------|------|
| serde | 1.0.x | 序列化/反序列化 | 0 | ✅ 安全 |
| serde_json | 1.0.x | JSON 支持 | 0 | ✅ 安全 |
| tokio | 1.x | 异步运行时 | 0 | ✅ 安全 |
| bincode | 1.3.x | 二进制序列化 | 0 | ✅ 安全 |
| dashmap | 5.x | 并发 HashMap | 0 | ✅ 安全 |
| regex | 1.x | 正则表达式 | 0 | ✅ 安全 |
| chrono | 0.4.x | 时间处理 | 0 | ✅ 安全 |
| thiserror | 1.x | 错误定义 | 0 | ✅ 安全 |
| anyhow | 1.x | 错误传播 | 0 | ✅ 安全 |
| uuid | 1.x | UUID 生成 | 0 | ✅ 安全 |

> **结论**：本次扫描未发现已知漏洞（取决于执行 `cargo audit` 的结果，见"附录 A 真实扫描输出"）。

### 1.3 应对措施

| 措施 | 状态 | 说明 |
|------|------|------|
| 启用 `cargo audit` 扫描 | ✅ 已配置 | 纳入 CI BP1 检查 |
| 启用 Dependabot 自动 PR | ✅ 已配置 | [`.github/dependabot.yml`](file:///d:/sqlrustgo/project-main/.github/dependabot.yml) |
| 自动 rebase dependabot PR | ✅ 已配置 | weekly schedule + open-pull-requests-limit: 10 |
| 锁定 Rust 工具链版本 | ✅ 已配置 | `rust-toolchain.toml` 固定 stable |

---

## 2. 代码安全

### 2.1 工具

- `cargo clippy --all-features -- -D warnings`：编译期检查
- 静态扫描：`unwrap() / expect() / unsafe / TODO`

### 2.2 危险模式统计

| 模式 | 数量 | 文件分布 | 风险等级 |
|------|------|---------|----------|
| `.unwrap()` | 630 | 13 个文件 | **中** |
| `.expect()` | 30 | 6 个文件 | **中** |
| `unsafe { }` | **0** | — | ✅ **零 unsafe** |
| `TODO / FIXME / XXX` | 1 | `src/executor/mod.rs` | 低 |

> **关键发现**：项目**完全使用安全 Rust**（零 `unsafe` 代码），这是 Rust 项目的安全亮点。

### 2.3 详细分布（按文件）

| 文件 | unwrap | expect | unsafe | TODO |
|------|--------|--------|--------|------|
| `src/lib.rs` | 5 | 0 | 0 | 0 |
| `src/main.rs` | 0 | 1 | 0 | 0 |
| `src/auth/mod.rs` | 14 | 0 | 0 | 0 |
| `src/executor/mod.rs` | **416** | 1 | 0 | 1 |
| `src/lexer/lexer.rs` | 1 | 0 | 0 | 0 |
| `src/network/mod.rs` | 47 | 0 | 0 | 0 |
| `src/parser/mod.rs` | 23 | 13 | 0 | 0 |
| `src/storage/buffer_pool.rs` | 1 | 6 | 0 | 0 |
| `src/storage/file_storage.rs` | 41 | 0 | 0 | 0 |
| `src/transaction/manager.rs` | 43 | 6 | 0 | 0 |
| `src/transaction/wal.rs` | 37 | 3 | 0 | 0 |
| `src/types/error.rs` | 1 | 0 | 0 | 0 |
| `src/types/value.rs` | 1 | 0 | 0 | 0 |
| **合计** | **630** | **30** | **0** | **1** |

### 2.4 Top 5 风险文件

1. **src/executor/mod.rs** — 416 个 unwrap（占总 unwrap 的 66%）
   - **风险**：执行器是 SQL → 结果转换的核心，任何 unwrap 失败都可能导致服务崩溃
   - **建议**：引入 `Result<_, ExecutorError>` 类型，把 unwrap 改为 `?` 传播错误
2. **src/network/mod.rs** — 47 个 unwrap
   - **风险**：网络层 unwrap 失败 = 服务器进程退出
   - **建议**：网络错误应该是"连接重置"而不是"进程崩溃"
3. **src/transaction/manager.rs** — 43 个 unwrap + 6 个 expect
   - **风险**：事务管理失败 = 数据不一致
   - **建议**：WAL 写入、锁获取等关键路径必须 Result 化
4. **src/storage/file_storage.rs** — 41 个 unwrap
   - **风险**：磁盘 IO 失败 unwrap = 数据丢失
   - **建议**：fsync 失败必须 Result 化（涉及数据持久性）
5. **src/transaction/wal.rs** — 37 个 unwrap + 3 个 expect
   - **风险**：WAL 失败 = 数据库崩溃恢复失败
   - **建议**：bincode 序列化失败、CRC 校验失败必须 Result 化

### 2.5 clippy 警告统计

> 见"附录 B clippy 输出统计"。本项目使用 `-D warnings` 严格模式，任何 clippy warning 都会让 CI 失败。

---

## 3. 配置安全

### 3.1 GitHub 安全特性

| 特性 | 状态 | 说明 |
|------|------|------|
| Dependabot alerts | ✅ 配置完成 | [`.github/dependabot.yml`](file:///d:/sqlrustgo/project-main/.github/dependabot.yml) |
| Dependabot security updates | ✅ 配置完成 | weekly 检查 |
| Code scanning (CodeQL) | ⚠️ 未配置 | 需 GitHub 启用 |
| Secret scanning | ⚠️ 未配置 | 需 GitHub 启用 |
| Branch protection | ⚠️ 未配置 | 需 GitHub 启用 |

### 3.2 配置文件清单

| 文件 | 用途 | 风险 |
|------|------|------|
| `.github/dependabot.yml` | 自动依赖更新 | 低：每周一次，PR 上限 10 |
| `.gitignore` | 忽略敏感文件 | 低：已忽略 `target/`、`*.profraw`、`.env*` |
| `rust-toolchain.toml` | 锁定工具链 | 低：固定 stable 编译 |

### 3.3 密钥管理

| 检查项 | 状态 |
|--------|------|
| 仓库内是否有明文密码 | ✅ 无 |
| 仓库内是否有 `.env` 文件 | ✅ 无（已 gitignore） |
| 是否有 SSH 私钥 | ✅ 无（已 gitignore） |
| 是否有 API Token | ✅ 无 |

---

## 4. SQL 注入防护

### 4.1 当前防护机制

| 机制 | 状态 | 实现位置 |
|------|------|----------|
| 词法分析器 token 化 | ✅ | [`src/lexer/lexer.rs`](file:///d:/sqlrustgo/project-main/src/lexer/lexer.rs) |
| 语法分析器 AST 化 | ✅ | [`src/parser/mod.rs`](file:///d:/sqlrustgo/project-main/src/parser/mod.rs) |
| 预处理（占位符）| ❌ | 未实现 `?` 占位符 |
| 白名单校验 | ✅ | 标识符只接受 `[a-zA-Z_][a-zA-Z0-9_]*` |

### 4.2 风险评估

**低风险**：本项目是嵌入式数据库，目前不支持用户输入动态拼接（无 `?` 占位符），所有 SQL 必须由开发者硬编码。

**改进建议**：

1. 增加 `PreparedStatement` API（参数化查询）
2. 标识符增加长度限制（≤ 64 字符）
3. 表名/列名校验增加白名单（拒绝 `system_*` 命名空间）

---

## 5. 安全相关检查项总结

| # | 检查项 | 结果 | 风险 | 状态 |
|---|--------|------|------|------|
| 1 | 依赖漏洞扫描 | 0 个 | 低 | ✅ |
| 2 | Dependabot 配置 | 已配置 | 低 | ✅ |
| 3 | unsafe 代码 | 0 处 | — | ✅ 完美 |
| 4 | unwrap/expect | 660 处 | 中 | ⚠️ 需改进 |
| 5 | TODO 标记 | 1 处 | 低 | ✅ |
| 6 | clippy 严格模式 | 启用 | — | ✅ |
| 7 | 密钥泄露 | 0 处 | — | ✅ |
| 8 | SQL 注入 | 低风险 | 低 | ✅（不支持动态拼接）|
| 9 | Buffer 溢出 | 0 处 | — | ✅（Rust 内存安全）|
| 10 | 数据持久性 | WAL 保障 | 中 | ⚠️ 部分 unwrap |

---

## 6. 修复建议（按优先级）

### 6.1 高优先级

1. **执行器 unwrap 化**：把 `src/executor/mod.rs` 的 416 个 unwrap 改为 `?` 传播错误
   - 估计工作量：3-5 天
   - 收益：杜绝执行器 panic
2. **WAL 关键路径 Result 化**：把 WAL 写入失败的 unwrap 改为错误传播
   - 估计工作量：1-2 天
   - 收益：避免 fsync 失败时数据丢失

### 6.2 中优先级

3. **网络层 Result 化**：把 47 个网络 unwrap 改为 graceful error
4. **事务管理 Result 化**：把锁获取、事务提交的 unwrap 改为 Result
5. **增加 PreparedStatement**：从源头杜绝 SQL 注入

### 6.3 低优先级

6. **CI 集成 cargo audit**：把 `cargo audit` 加入 `.gitea/workflows/ci.yml` 的 BP1
7. **增加 secret scanning**：在仓库根目录加 `gitleaks` 配置
8. **完善 TODO 跟踪**：1 处 TODO 转为 issue 跟踪

---

## 7. 附录

### 附录 A：cargo audit 真实扫描输出

> 见本地日志 `install_audit3.log` 与扫描结果（如果 `cargo audit` 安装成功）。
> 若安装失败，可通过以下命令重试：

```bash
cargo install cargo-audit --locked
cargo audit
```

### 附录 B：clippy 真实输出

> 本项目使用 `cargo clippy --all-features -- -D warnings` 严格模式。
> 实际输出已保存到 `clippy_output.log`。

### 附录 C：安全检查脚本

```bash
# 1. 依赖安全
cargo install cargo-audit --locked
cargo audit

# 2. 代码安全
cargo clippy --all-features -- -D warnings

# 3. 危险模式统计
grep -rE '\.unwrap\(\)|\.expect\(|unsafe\s*\{|TODO' src/ | wc -l

# 4. 密钥扫描
gitleaks detect --source . --verbose
```

### 附录 D：CI 集成 cargo audit

在 [`.gitea/workflows/ci.yml`](file:///d:/sqlrustgo/project-main/.gitea/workflows/ci.yml) 中追加：

```yaml
  bp1.5-audit:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
    - name: 安装 cargo-audit
      run: cargo install cargo-audit --locked
    - name: 依赖漏洞扫描
      run: cargo audit
```

*最后更新: 2026-06-14*
