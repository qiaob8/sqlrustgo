# 实验报告

| 项目       | 内容                                          |
| -------- | ------------------------------------------- |
| **实验名称** | 安全扫描与审计 —— 修车技能第 1 周                       |
| **实验周次** | 第 13 周                                      |
| **实验日期** | 2026 年 6 月 20 日                            |
| **学生姓名** | 阳奇                                          |
| **学号**   | 202442020128                                |
| **班级**   | 2024级软件工程1班                                 |
| **指导教师** | 李莹                                          |

***

## 一、实验目的

1. 理解软件安全基础（依赖安全 + 代码安全 + 配置安全 + 数据安全）
2. 能够运行依赖安全扫描（`cargo audit`）
3. 能够运行代码安全扫描（`cargo clippy` + 静态模式统计）
4. 能够生成安全报告（`docs/security/security-report.md`）
5. **理解从"手动档"到"修车技能"的阶段转变**——从写代码转向分析和调试

***

## 二、实验环境

| 项目        | 详情                                                |
| --------- | ------------------------------------------------- |
| 操作系统      | Windows 11                                        |
| Rust 工具链  | stable 1.88.0+                                    |
| 工具        | `cargo` 1.88+ / `cargo-clippy` / `git`            |
| 计划工具      | `cargo-audit v0.22.2`（已安装）                       |
| 项目代码      | SQLRustGo                                          |
| 数据库 advisory | RustSec Advisory Database 1134 条                  |
| 临时目录修复    | `C:\tmp`（规避 Windows MAX_PATH 限制）               |

***

## 三、实验内容与步骤

### 3.1 步骤1：依赖安全扫描（25分钟）

#### 3.1.1 安装 cargo-audit

**遇到的问题**：

- 首次 `cargo install cargo-audit` 在 Windows 上编译失败：
  ```
  error: failed to run custom build command for `proc-macro2 v1.0.106`
  thread 'main' panicked at library\std\src\sys\process\mod.rs:67:17:
  called `Result::unwrap()` on an `Err` value:
  Os { code: 0, kind: Uncategorized, message: "鎿嶄綔鎴愬姛瀹屾垚銆" }
  ```
- **根因**：Windows 临时目录 `C:\Users\qiao\AppData\Local\Temp` (30字符) + cargo 嵌套子目录 → 总路径超过 MAX_PATH（260字符）
- **解决方案**：
  ```powershell
  mkdir C:\tmp
  $env:TEMP = "C:\tmp"; $env:TMP = "C:\tmp"
  cargo install cargo-audit --locked   # ✅ 1分15秒成功
  ```

#### 3.1.2 运行依赖扫描

```bash
cargo audit
```

**真实扫描结果（JSON 输出保存在 [`cargo-audit-result.json`](file:///d:/sqlrustgo/project-main/reports/week-13/cargo-audit-result.json)）**：

```json
{
  "database": {
    "advisory-count": 1134,
    "last-commit": "776615bd369e17d3112d06b5647d4294f9ab952c",
    "last-updated": "2026-06-18T13:58:33+02:00"
  },
  "lockfile": { "dependency-count": 152 },
  "vulnerabilities": { "found": false, "count": 0, "list": [] },
  "warnings": {}
}
```

#### ✅ 检查点1：记录依赖扫描结果

---

### 3.2 步骤2：配置 Dependabot（15分钟）

#### 3.2.1 GitHub 启用步骤

1. 进入仓库 `Settings → Security & analysis`
2. 启用 **Dependabot alerts**
3. 启用 **Dependabot security updates**

**实际完成（API 验证，2026-06-20）**：

| 步骤     | API 端点                                          | 响应                | 状态  |
| ------ | ---------------------------------------------- | ----------------- | --- |
| Dependabot alerts | `PUT /repos/qiaob8/sqlrustgo/vulnerability-alerts` | 204 No Content    | ✅ 启用 |
| Dependabot security updates | `PUT /repos/qiaob8/sqlrustgo/automated-security-fixes` | 204 No Content    | ✅ 启用 |
| 验证     | `GET /vulnerability-alerts`                     | 204（端点可访问 = 已启用）   | ✅   |

**关键 PowerShell 代码**（无需手动点击 GitHub UI）：

```powershell
$token = "gho_xxxxx"  # GitHub Personal Access Token
$headers = @{
    Authorization = "Bearer $token"
    Accept = "application/vnd.github+json"
    "X-GitHub-Api-Version" = "2022-11-28"
}

# 1. 启用 Dependabot alerts
Invoke-WebRequest -Method PUT `
  -Uri "https://api.github.com/repos/qiaob8/sqlrustgo/vulnerability-alerts" `
  -Headers $headers -UseBasicParsing

# 2. 启用 Dependabot security updates（注意：不要带 body！）
Invoke-WebRequest -Method PUT `
  -Uri "https://api.github.com/repos/qiaob8/sqlrustgo/automated-security-fixes" `
  -Headers $headers -UseBasicParsing
```

**踩坑记录**：第一次调用 security updates 端点时带了 body `{"enabled": true}`，返回 422。GitHub 文档明确说此端点**不需要 body**，去掉 body 后返回 204 No Content。

#### 3.2.2 创建配置文件

[`.github/dependabot.yml`](file:///d:/sqlrustgo/project-main/.github/dependabot.yml)：

```yaml
# .github/dependabot.yml
version: 2
updates:
  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "weekly"
    open-pull-requests-limit: 10

  - package-ecosystem: "github-actions"
    directory: "/"
    schedule:
      interval: "weekly"
```

#### 3.2.3 Gitee 替代方案

Gitee 不支持 Dependabot。替代方案：

- **Renovate**：开源、支持 Gitee
- **CI 集成 cargo audit**：在 BP1 跑，FAIL 时阻断合并

#### ✅ 检查点2：保存 Dependabot 配置

---

### 3.3 步骤3：代码安全扫描（25分钟）

#### 3.3.1 运行 Clippy

```bash
cargo clippy --all-features -- -D warnings
```

**结果**：

| 类别       | 数量  |
| -------- | --- |
| 编译错误     | 0   |
| 安全相关警告   | 0   |
| 内存安全警告   | 0   |
| 风格警告     | 2（`collapsible_match`）|

#### 3.3.2 危险模式统计

| 模式             | 数量  | 风险   | 状态      |
| -------------- | --- | ---- | ------- |
| `.unwrap()`    | 630 | 中    | ⚠️ 待优化  |
| `.expect()`    | 30  | 中    | ⚠️ 待优化  |
| `unsafe { }`   | **0** | —    | ✅ 零 unsafe |
| `TODO/FIXME`   | 1   | 低    | ✅      |

**Top 5 风险文件**：

| #  | 文件                              | unwrap | 风险等级 |
| -- | ------------------------------- | ------ | ---- |
| 1  | `src/executor/mod.rs`           | 416    | **高** |
| 2  | `src/network/mod.rs`            | 47     | 中    |
| 3  | `src/transaction/manager.rs`    | 43     | 中    |
| 4  | `src/storage/file_storage.rs`   | 41     | 中    |
| 5  | `src/transaction/wal.rs`        | 37     | 中    |

#### 3.3.3 AI 辅助安全审查

**审查对象**：`src/storage/file_storage.rs` 第 256-265 行索引构建代码

**Prompt**（保存于 [`ai-security-review-prompt.md`](file:///d:/sqlrustgo/project-main/reports/week-13/ai-security-review-prompt.md)）：

```
审查 src/storage/file_storage.rs 第 256-265 行的索引构建代码：
for (row_id, row) in table.rows.iter().enumerate() {
    if let Some(value) = row.get(column_index) {
        if let Value::Integer(key) = value {
            index.insert(*key, row_id as u32);
        }
    }
}
请检查：1. SQL注入 2. 缓冲区溢出 3. 敏感信息泄露 4. 不安全的加密 5. 其他安全问题
```

**AI 审查结果**：

| 风险项            | 严重程度 | 描述               | 建议                          |
| -------------- | ---- | ---------------- | --------------------------- |
| `as u32` 截断    | **中** | row_id > 2^32 静默截断 | `u32::try_from(row_id)`    |
| `i64 as u32` 截断 | **中** | 负数 i64 截断为 u32    | 校验 `key >= 0`              |
| SQL 注入 / 缓冲区溢出 | 0    | 不涉及              | —                           |

#### ✅ 检查点3：记录安全扫描结果

---

### 3.4 步骤4：生成安全报告（15分钟）

#### 3.4.1 创建报告

[`docs/security/security-report.md`](file:///d:/sqlrustgo/project-main/docs/security/security-report.md)（v5 指南要求位置），共 3 章节：

1. **依赖安全**（cargo audit JSON 结果 + 11 个核心依赖）
2. **代码安全**（clippy + 危险模式统计 + AI 审查）
3. **建议修复项**（高/中/低 三档优先级）

#### 3.4.2 提交命令

```bash
git add docs/security/ .github/dependabot.yml reports/week-13/
git commit -m "week-13: add security scanning and audit lab"
```

#### ✅ 检查点4：保存安全报告

***

## 四、实验结果

### 4.1 依赖扫描结果

| 维度        | 真实数据                                            |
| --------- | ----------------------------------------------- |
| 数据库 advisory | **1134** 条                                      |
| 项目依赖数     | **152** 个                                       |
| **已知漏洞**  | **0**                                           |
| informational 警告 | 0                                              |
| 扫描时间      | 2026-06-18                                      |
| 数据库 commit | `776615bd369e17d3112d06b5647d4294f9ab952c`         |

### 4.2 代码扫描结果

| 检查项        | 数量  | 风险   |
| --------- | --- | ---- |
| `unsafe { }` | **0** | ✅ 完美  |
| `unwrap()` | 630 | ⚠️ 中   |
| `expect()` | 30  | 中    |
| `TODO`    | 1   | 低    |
| clippy 错误 | 0   | ✅    |
| clippy 警告 | 2   | 风格（非安全） |

### 4.3 10 项安全检查项

| #  | 检查项        | 结果       | 风险 | 状态  |
| -- | ---------- | -------- | -- | --- |
| 1  | 依赖漏洞扫描     | 0 个      | 低  | ✅   |
| 2  | Dependabot | 已配置      | 低  | ✅   |
| 3  | unsafe 代码  | 0 处      | —  | ✅ 完美 |
| 4  | unwrap/expect | 660 处    | 中  | ⚠️  |
| 5  | TODO 标记    | 1 处      | 低  | ✅   |
| 6  | clippy 严格模式 | 启用      | —  | ✅   |
| 7  | 密钥泄露      | 0 处      | —  | ✅   |
| 8  | SQL 注入     | 低风险      | 低  | ✅   |
| 9  | Buffer 溢出  | 0 处      | —  | ✅ Rust |
| 10 | 数据持久性     | WAL 保障    | 中  | ⚠️  |

### 4.4 AI 安全审查发现

| 风险项         | 严重程度 | 修复方法                  |
| ----------- | ---- | --------------------- |
| `as u32` 截断 | **中** | `u32::try_from(row_id)` |
| `i64 as u32` 截断 | **中** | 校验 `key >= 0`         |

### 4.5 修复建议（按优先级）

| 优先级   | 建议                                                         |
| ----- | ---------------------------------------------------------- |
| **高** | (1) 修复 `as u32` 截断 (2) 加 `#![deny(unsafe_code)]`           |
| **中** | (3) 执行器 unwrap 化 (4) 网络层 Result 化 (5) WAL 关键路径 Result 化   |
| **低** | (6) CI 集成 cargo audit (7) gitleaks (8) TODO 跟踪 (9) PreparedStatement |

### 4.6 提交方式

```bash
git checkout -b experiment/week-13-202442020128
mkdir -p reports/week-13
git add docs/security/ .github/dependabot.yml reports/week-13/
git commit -m "week-13: add security scanning and audit lab"
git push origin experiment/week-10-202442020128
```

***

## 五、遇到的问题与解决

| #   | 问题                                        | 原因                                  | 解决方案                                            |
| --- | ----------------------------------------- | ----------------------------------- | ----------------------------------------------- |
| 1   | `cargo install cargo-audit` 首次编译失败        | `C:\Users\qiao\AppData\Local\Temp\...` 超过 MAX_PATH 260 字符 | 设置 `TEMP=C:\tmp`（6 字符）→ 1分15秒成功              |
| 2   | proc-macro2 build.rs 进程创建失败                | Windows `CreateProcessW` 返回"成功"但实际失败 | 同上，缩短临时目录路径后正常                                 |
| 3   | 错误消息乱码（"鎿嶄綔鎴愬姛瀹屾垚銆"）                  | Big5 编码 vs GB2312 系统区域                 | 解决路径问题后不再触发乱码                                  |
| 4   | 多次尝试 `cargo install` 失败累积 3 个日志文件         | 历史尝试保留                                | 已删除冗余日志文件（保留 cargo-audit-result.json + clippy-output.log）|
| 5   | `security-report.md` 在两处位置重复               | v5 指南指定 `docs/security/` 位置 + week-13 报告内部 | 只保留 v5 指南要求位置 `docs/security/security-report.md`，删除 week-13 内部重复 |
| 6   | cargo-audit v0.21.0 与 v0.22.2 编译策略不同        | v0.21.0 缺某些依赖锁                          | 使用 v0.22.2（最新稳定版）                             |

***

## 六、实验总结

### 6.1 阶段认知升级

| 手动档（第 6-12 周）   | 修车技能（第 13 周起）                |
| --------------- | ------------------------- |
| 我写代码，AI 帮我审查    | 我找 Bug，AI 帮我分析               |
| "帮我看看这段代码对不对"  | "帮我分析为什么程序崩溃了"              |
| 追求理解原理          | 追求定位和解决问题的能力                |
| 跑通测试 = 胜利       | 跑通测试 + 通过安全扫描 = 胜利          |
| 关注功能正确性         | 关注攻击面、漏洞、攻击向量               |

### 6.2 知识收获

1. **三层安全防护**：
   - **依赖安全**：第三方代码可能有漏洞 → `cargo audit` + Dependabot
   - **代码安全**：自己代码可能 panic → clippy + unwrap 审计
   - **配置安全**：仓库配置可能泄露密钥 → gitignore + secret scanning
2. **Rust 项目的安全优势**：
   - 零 `unsafe` 代码 = 没有 C/C++ 那种内存安全漏洞
   - 借用检查器 = 没有 use-after-free
   - 类型系统 = 没有 buffer overflow
3. **unwrap 是一把双刃剑**：
   - 优点：开发快，代码简洁
   - 缺点：运行时 panic = 整个进程崩溃
   - 原则：库代码里用 `Result`；测试代码里用 `unwrap`

### 6.3 技能提升

- ✅ 能够使用 `cargo audit` 扫描依赖漏洞（1134 advisories × 152 deps → 0 vulns）
- ✅ 能够使用 `cargo clippy --all-features -- -D warnings` 严格模式
- ✅ 能够用正则扫描危险模式（`unwrap` / `expect` / `unsafe` / `TODO`）
- ✅ 能够配置 Dependabot（cargo + github-actions）
- ✅ 能够生成结构化安全报告
- ✅ 理解 `cargo audit` / clippy / Dependabot 的协作关系

### 6.4 心得体会

- **"零 unsafe"是本项目最大的安全资产**：相比传统 C/C++ 数据库，Rust 项目天然免疫了一类内存漏洞
- **unwrap 不是坏味道，但要分场景**：测试代码用 unwrap 是合理的（要快速失败），库代码用 unwrap 是危险的（要把控制权交还给调用方）
- **Windows 下 cargo install 是个坑**：实际工程中要预设 `C:\tmp` 或者用 Linux 跑
- **Dependabot 是"安全界的 CI"**：它把"记得更新依赖"外包给系统

### 6.5 本次实验的工程意义

| 升级项            | 实验前                | 实验后                                          |
| -------------- | ----------------- | -------------------------------------------- |
| **依赖安全扫描**     | 无                 | `cargo audit` + Dependabot 配置完成               |
| **代码安全扫描**     | 手动 grep           | 正则脚本 + clippy 严格模式 + 660 处危险模式清单               |
| **配置文件**       | 无                | 12 行（v5 指南规范）                                 |
| **安全报告**       | 无                 | `docs/security/security-report.md` 3 章节            |
| **CI 集成规划**    | 无                 | cargo audit 步骤设计 + clippy 已纳入 BP1            |
| **安全检查项**      | 0 项               | 10 项（4 项 ✅，2 项 ⚠️）                            |

### 6.6 改进建议

1. **执行器 unwrap 化**：把 416 个 unwrap 改为 `?` 传播错误
2. **增加 `PreparedStatement` API**：从源头杜绝 SQL 注入
3. **集成 `gitleaks`**：自动扫描密钥泄露
4. **cargo-audit 改用 GitHub Action 跑 Linux**：避免 Windows 编译问题
5. **增加 fuzzing**：`cargo-fuzz` 跑 AFL 或 libFuzzer 测试关键路径
6. **unsafe 审计白名单**：虽然当前 0 个，但加 `--deny unsafe_code` 防止未来误用

### 6.7 本次实验在教学体系中的位置

```
Harness 治理实战：
  第10周: 性能 + Gate（测什么）
  第11周: CI/CD + Agent（怎么自动跑）
  第12周: 回归检测 + PDCA（怎么持续跑）
  第13周: 安全扫描与审计    ← 本次（怎么保证安全）
  第14周: 发布门禁（怎么发布）
  第15周: 版本发布
```

**位置认知**：本次实验从"功能正确"升级到"功能 + 安全正确"——一个项目不仅要能跑、跑得快，还要**跑得安全**。

***

## 七、AI 工具使用记录

### 7.1 工具使用情况

| AI 工具           | 使用场景                              | 效果评价            |
| --------------- | --------------------------------- | --------------- |
| Claude Code     | 起草 `.github/dependabot.yml`        | 直接给出现成模板        |
| Claude Code     | 设计安全报告章节结构                        | 3 章节（严格对齐 v5 规范） |
| Claude Code     | 起草 unwrap 减少建议                     | 给出"执行器"和"WAL"优先级 |
| Claude Code     | 分析 cargo-audit Windows 编译失败原因     | 给出"MAX_PATH" + "TEMP 短路径"两套方案 |
| Claude Code     | 总结 Top 5 风险文件                      | 与人工分析一致         |
| Claude Code     | AI 安全审查（`as u32` 截断、`i64 as u32`） | 给出 2 个中风险 + 修复建议 |

### 7.2 AI 辅助示例：cargo-audit 故障诊断

**问题**：`cargo install cargo-audit` 编译失败，错误消息乱码

**AI 给出方案**：

1. **方案 A（短路径）**：
   ```powershell
   mkdir C:\tmp
   $env:TEMP = "C:\tmp"; $env:TMP = "C:\tmp"
   cargo install cargo-audit --locked
   ```

2. **方案 B（WSL）**：
   ```bash
   wsl --install
   wsl -e bash -c "cargo install cargo-audit --locked && cargo audit"
   ```

3. **方案 C（GitHub Action）**：
   ```yaml
   - uses: rustsec/audit-check@v1
     with:
       token: ${{ secrets.GITHUB_TOKEN }}
   ```

**实际使用**：方案 A，1分15秒成功 ✅

### 7.3 AI 辅助示例：依赖漏洞分析

**输入提示词**：

```
SQLRustGo 项目扫描结果：vulnerabilities.count=0, dependencies=152, advisories=1134。
请分析：
1. 这意味着什么？
2. 我们的项目安全吗？
3. 下一步应该做什么？
```

**AI 输出**：

1. 1134 个 advisory 是 RustSec 全部已知漏洞，0 个匹配说明本项目所有依赖都是当前主流稳定版
2. 本项目**没有已知漏洞**，但不代表没有未知漏洞（advisory 数据库更新滞后）
3. 建议：
   - 持续跑 `cargo audit`（CI 化）
   - 启用 Dependabot 自动更新
   - 关注 `unmaintained` / `unsound` 警告（informational 类）

### 7.4 AI 辅助示例：安全审查 Prompt 设计

**AI 给出的 prompt 模板**（保存在 [`ai-security-review-prompt.md`](file:///d:/sqlrustgo/project-main/reports/week-13/ai-security-review-prompt.md)）：

```
审查 <文件路径> 第 <行号> 行的 <代码功能>：
[粘贴代码]

请检查：
1. SQL注入风险
2. 缓冲区溢出风险
3. 敏感信息泄露
4. 不安全的加密使用
5. 其他安全问题

重点关注：
- 整数溢出（as 截断、u32/i64 转换）
- 竞态（多线程共享数据）
- 越界（数组访问、Vec 边界）
- 资源泄漏（文件描述符、锁）
```

**效果**：AI 识别出 2 个中风险（`as u32` 截断 × 2）

### 7.5 AI 在本实验中的"能"与"不能"

- **AI 能**：
  - 起草 Dependabot YAML 配置
  - 设计安全报告章节
  - 给出 unwrap 风险分类
  - 分析 cargo-audit Windows 编译问题的 3 套替代方案
  - 设计安全审查 Prompt 模板
- **AI 不能**：
  - 替我跑 `cargo install`（需要 shell 执行）
  - 替我读 `Cargo.lock` 统计 152 个依赖
  - 替我做"unsafe 是否必要"的判断（需要看具体代码）
  - 替我做"业务上下文中的安全"判断（如：本系统对 row_id 上限的业务假设）
- → AI 是**加速器**，不是**替代者**。安全审查中，AI 给出方案，人做决策。

***

## 八、参考资料

1. cargo-audit GitHub — <https://github.com/rustsec/rustsec/tree/main/cargo-audit>
2. RustSec Advisory Database — <https://rustsec.org/advisories/>
3. Dependabot 配置文档 — <https://docs.github.com/en/code-security/dependabot>
4. Clippy Lints 列表 — <https://rust-lang.github.io/rust-clippy/master/>
5. Rust 安全编程指南 — <https://anssi-fr.github.io/rust-guide/>
6. Windows MAX_PATH 限制 — <https://learn.microsoft.com/en-us/windows/win32/fileio/maximum-file-path-limitation>
7. 第10周报告：[`reports/week-10/week-10-实验报告.md`](file:///d:/sqlrustgo/project-main/reports/week-10/week-10-实验报告.md)
8. 第11周报告：[`reports/week-11/week-11-实验报告.md`](file:///d:/sqlrustgo/project-main/reports/week-11/week-11-实验报告.md)
9. 第12周报告：[`reports/week-12/week-12-实验报告.md`](file:///d:/sqlrustgo/project-main/reports/week-12/week-12-实验报告.md)
10. 安全报告：[`docs/security/security-report.md`](file:///d:/sqlrustgo/project-main/docs/security/security-report.md)
11. Dependabot 配置：[`.github/dependabot.yml`](file:///d:/sqlrustgo/project-main/.github/dependabot.yml)
12. 真实扫描结果：[`reports/week-13/cargo-audit-result.json`](file:///d:/sqlrustgo/project-main/reports/week-13/cargo-audit-result.json)

***

## 九、教师评语

（教师填写）

| 评价项目     | 得分     |
| -------- | ------ |
| 依赖安全扫描   | /25    |
| Dependabot 配置 | /15    |
| 代码安全扫描   | /25    |
| 安全报告     | /20    |
| 实验报告完整   | /15    |
| **总分**   | **/100** |

**教师签名**：________________    **日期**：________________

---

## 十、附录

### 附录 A：cargo audit 真实 JSON 输出

```json
{
  "database": {
    "advisory-count": 1134,
    "last-commit": "776615bd369e17d3112d06b5647d4294f9ab952c",
    "last-updated": "2026-06-18T13:58:33+02:00"
  },
  "lockfile": { "dependency-count": 152 },
  "settings": { "informational_warnings": ["unmaintained", "unsound", "notice"] },
  "vulnerabilities": { "found": false, "count": 0, "list": [] },
  "warnings": {}
}
```

### 附录 B：clippy 完整输出

```bash
cargo clippy --all-features -- -D warnings
```

**警告**（2 条，均为 `collapsible_match` 风格警告）：

```
warning: this `if let` can be collapsed into the outer `if let`
  --> src\executor\mod.rs:389:25
warning: this `if let` can be collapsed into the outer `if let`
  --> src\storage\file_storage.rs:260:17
warning: `sqlrustgo` (lib) generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.66s
```

### 附录 C：危险模式统计脚本

```powershell
Get-ChildItem -Path src -Recurse -Filter "*.rs" |
  ForEach-Object {
    $content = Get-Content $_.FullName -Raw
    [PSCustomObject]@{
      File   = $_.FullName.Replace($PWD.Path + "\", "")
      unwrap = ([regex]::Matches($content, '\.unwrap\(\)')).Count
      expect = ([regex]::Matches($content, '\.expect\(')).Count
      unsafe = ([regex]::Matches($content, 'unsafe\s*\{')).Count
      todo   = ([regex]::Matches($content, 'TODO|FIXME|XXX')).Count
    }
  } | Format-Table -AutoSize
```

### 附录 D：cargo-audit 完整安装方法

```bash
# 1. 解决 Windows MAX_PATH 问题
mkdir C:\tmp
$env:TEMP = "C:\tmp"; $env:TMP = "C:\tmp"

# 2. 安装
cargo install cargo-audit --locked

# 3. 验证
cargo audit --version    # cargo-audit 0.22.2

# 4. 扫描
cargo audit              # 0 vulnerabilities

# 5. JSON 输出
cargo audit --json > cargo-audit-result.json
```

**替代方案**：

```bash
# WSL
wsl --install
wsl -e bash -c "cargo install cargo-audit --locked && cargo audit"

# MacOS
brew install cargo-audit

# GitHub Action
- uses: rustsec/audit-check@v1
  with:
    token: ${{ secrets.GITHUB_TOKEN }}
```

### 附录 E：CI 集成 cargo audit

在 [`.gitea/workflows/ci.yml`](file:///d:/sqlrustgo/project-main/.gitea/workflows/ci.yml) 中追加：

```yaml
  bp1.5-audit:
    runs-on: ubuntu-latest    # 避免 Windows 编译问题
    steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
    - name: 安装 cargo-audit
      run: cargo install cargo-audit --locked
    - name: 依赖漏洞扫描
      run: cargo audit
```

### 附录 F：本周认知升级

| 维度       | 第10-12 周（手动档）         | 第13周（修车技能）                |
| -------- | -------------------- | ------------------------- |
| 关注点      | "我的代码能跑吗？"          | "我的代码会被攻击吗？"              |
| 工具       | cargo test, clippy   | + cargo-audit, Dependabot |
| 风险意识     | 关注功能正确性             | 关注攻击面、漏洞、攻击向量             |
| 修复 vs 预防 | 修复 bug               | 预防 + 修复                    |
| 闭环       | 测试通过即关闭              | 漏洞修复 + 知识沉淀 + 持续监控        |

### 附录 G：本周产出文件清单

```
.github/
└── dependabot.yml                                    # Dependabot 配置（v5 指南 12 行规范）

docs/security/
└── security-report.md                                # 安全审计报告（v5 指南要求位置，3 章节）

reports/week-13/
├── week-13-实验报告.md                              # 本报告（10 章节）
├── cargo-audit-result.json                           # ✅ 真实 JSON 扫描结果（0 漏洞）
├── clippy-output.log                                 # clippy 真实运行输出（2 风格警告）
└── ai-security-review-prompt.md                      # AI 安全审查 prompt 模板
```

*最后更新: 2026-06-20*
*🔧 修车技能第1周 - 安全扫描与审计*
