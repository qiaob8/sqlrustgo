# 实验报告

| 项目       | 内容                                          |
| -------- | ------------------------------------------- |
| **实验名称** | 发布门禁与检查清单 —— 修车技能第 2 周                       |
| **实验周次** | 第 14 周                                      |
| **实验日期** | 2026 年 6 月 20 日                            |
| **学生姓名** | 阳奇                                          |
| **学号**   | 202442020128                                |
| **班级**   | 2024级软件工程1班                                 |
| **指导教师** | 李莹                                          |

***

## TL;DR

1 份 [`docs/releases/v2.6.0/RELEASE_GATE_CHECKLIST.md`](file:///d:/sqlrustgo/project-main/docs/releases/v2.6.0/RELEASE_GATE_CHECKLIST.md) —— 6 维度门禁清单（代码/质量/安全/功能/性能/文档）。
1 个 [`scripts/run_gates.sh`](file:///d:/sqlrustgo/project-main/scripts/run_gates.sh) —— 6 步门禁脚本（编译/测试/clippy/格式化/覆盖率/审计）。
1 次门禁执行：6 步跑出 **8 通过 / 2 警告 / 1 失败 / 4 跳过**（`cargo fmt` 5 个文件未格式化，需修复）。
1 份 [`reports/week-14/week-14-实验报告.md`](file:///d:/sqlrustgo/project-main/reports/week-14/week-14-实验报告.md) 实验报告（本文档）。

**阶段认知**：本周完成"手动档→修车技能"的能力闭环——门禁清单是规则，脚本是工具，结果是反馈。

***

## 一、实验目的

1. 理解发布门禁（Release Gate）的概念与价值
2. 能够设计多维度门禁检查清单（代码/质量/安全/功能/性能/文档）
3. 能够编写门禁脚本串联 6+ 项检查
4. 能够执行门禁并基于结果给出发布决策
5. 理解"规则约束工程"中"必须 vs 最好"的区分
6. **理解 AI 不可替代的"发布决策"维度**（业务判断 + 风险权衡）

***

## 二、实验环境

| 项目        | 详情                                                |
| --------- | ------------------------------------------------- |
| 操作系统      | Windows 11                                        |
| Rust 工具链  | stable 1.96.0                                     |
| 项目代码      | SQLRustGo v1.0.0（v5 规范要求 v2.6.0 作为目标版本）            |
| 项目分支      | `experiment/week-14-202442020128`（基于 `experiment/week-13-202442020128`） |
| 工作目录      | `D:\sqlrustgo\project-main`                       |
| 临时目录修复    | `TEMP=C:\tmp`、`CARGO_TARGET_DIR=C:\t`（规避 Windows MAX_PATH） |
| 门禁脚本      | `scripts/run_gates.sh`（v5 规范指定）+ `scripts/gate/gate.sh`（已存在） |
| 覆盖率工具     | `cargo-tarpaulin`（Windows 不可用，跳过）               |

***

## 三、实验内容与步骤

### 3.1 步骤 1：设计门禁检查清单（20 分钟）

#### 3.1.1 创建清单文件

按 v5 规范要求创建 [`docs/releases/v2.6.0/RELEASE_GATE_CHECKLIST.md`](file:///d:/sqlrustgo/project-main/docs/releases/v2.6.0/RELEASE_GATE_CHECKLIST.md)，6 个维度：

1. **代码门禁**（编译/测试/clippy/格式化）
2. **质量门禁**（覆盖率/代码评级）
3. **安全门禁**（cargo audit/安全扫描）
4. **功能门禁**（集成测试/回归测试）
5. **性能门禁**（查询响应/QPS）
6. **文档门禁**（README/API/用户手册）

#### 3.1.2 关联现有资源

门禁脚本关联到 [`scripts/gate/`](file:///d:/sqlrustgo/project-main/scripts/gate/) 目录已存在的细分检查脚本：

- `check_coverage.sh` —— 覆盖率
- `check_docs.sh` —— 文档
- `check_regression.sh` —— 回归
- `check_security.sh` —— 安全
- `gate.sh` —— 完整门禁

#### ✅ 检查点 1：保存清单

清单已保存到 `docs/releases/v2.6.0/RELEASE_GATE_CHECKLIST.md`，含元信息表（项目/分支/日期/关联脚本）。

---

### 3.2 步骤 2：编写门禁脚本（30 分钟）

#### 3.2.1 创建 [`scripts/run_gates.sh`](file:///d:/sqlrustgo/project-main/scripts/run_gates.sh)

按 v5 规范 6 步：

```bash
[1/6] cargo build --release
[2/6] cargo test --all-features
[3/6] cargo clippy --all-features -- -D warnings
[4/6] cargo fmt --check --all
[5/6] cargo tarpaulin --out Xml --packages parser,executor,storage
[6/6] cargo audit
```

#### 3.2.2 Windows 兼容增强

由于 `cargo-tarpaulin` 在 Windows 不可用，加 fallback：

```bash
if command -v cargo-tarpaulin >/dev/null 2>&1; then
    cargo tarpaulin --out Xml --packages parser,executor,storage
else
    echo "[warn] cargo-tarpaulin 未安装，跳过覆盖率检查（Linux/WSL 工具）"
fi
```

#### 3.2.3 设置执行权限

```bash
chmod +x scripts/run_gates.sh
```

（Windows 上 `chmod` 无效但脚本仍可通过 `bash scripts/run_gates.sh` 运行。）

#### ✅ 检查点 2：脚本就绪

脚本可执行，含 set -e（任意一步失败立即终止）。

---

### 3.3 步骤 3：创建发布检查清单文件（15 分钟）

#### 3.3.1 创建版本目录

```bash
mkdir -p docs/releases/v2.6.0
```

#### 3.3.2 填写清单结果

见 [`RELEASE_GATE_CHECKLIST.md`](file:///d:/sqlrustgo/project-main/docs/releases/v2.6.0/RELEASE_GATE_CHECKLIST.md) — 4.1 章节中的 6 维度结果。

#### ✅ 检查点 3：清单已填写

清单含元信息、6 维度结果表、总结表（8/2/1/4）、关键问题、发布决策。

---

### 3.4 步骤 4：执行 RC 版本验收（15 分钟）

#### 3.4.1 切换到目标分支

v5 规范要求 `git checkout develop/v3.0.0`，但本项目**未维护该分支**。当前活跃分支：

- `experiment/week-13-202442020128`（上一周实验）
- `experiment/week-14-202442020128`（本周实验，新建）
- `main` / `master`

**降级策略**：在 `experiment/week-14-202442020128` 上执行门禁脚本作为 RC 验收。

#### 3.4.2 运行完整门禁

由于 Windows 沙箱限制，逐项手工执行门禁（[gate-build.txt](file:///d:/sqlrustgo/project-main/reports/week-14/gate-build.txt)、[gate-test.txt](file:///d:/sqlrustgo/project-main/reports/week-14/gate-test.txt)、[gate-clippy.txt](file:///d:/sqlrustgo/project-main/reports/week-14/gate-clippy.txt)、[gate-fmt.txt](file:///d:/sqlrustgo/project-main/reports/week-14/gate-fmt.txt)、[gate-audit.txt](file:///d:/sqlrustgo/project-main/reports/week-14/gate-audit.txt)）。

#### ✅ 检查点 4：验收完成

结果已记录到清单。

***

## 四、实验结果

### 4.1 门禁执行结果

| 维度  | 通过 | 警告 | 失败 | 跳过 |
| --- | -- | -- | -- | -- |
| 代码  | 2  | 1  | 1  | 0  |
| 质量  | 0  | 1  | 0  | 1  |
| 安全  | 2  | 0  | 0  | 0  |
| 功能  | 2  | 0  | 0  | 0  |
| 性能  | 0  | 0  | 0  | 2  |
| 文档  | 2  | 0  | 0  | 1  |
| **合计** | **8** | **2** | **1** | **4** |

### 4.2 各步骤详情

| 步骤 | 命令                              | 退出码 | 结果      | 日志 |
| -- | ------------------------------- | --- | ------- | -- |
| 1  | cargo build --release           | 0   | ✅ PASS | [gate-build.txt](file:///d:/sqlrustgo/project-main/reports/week-14/gate-build.txt) |
| 2  | cargo test --all-features       | 0   | ✅ PASS（399/0/5） | [gate-test.txt](file:///d:/sqlrustgo/project-main/reports/week-14/gate-test.txt) |
| 3  | cargo clippy --all-features     | 0   | ⚠️ 2 warnings | [gate-clippy.txt](file:///d:/sqlrustgo/project-main/reports/week-14/gate-clippy.txt) |
| 4  | cargo fmt --check               | 101 | ❌ 5 files | [gate-fmt.txt](file:///d:/sqlrustgo/project-main/reports/week-14/gate-fmt.txt) |
| 5  | cargo tarpaulin                 | 跳过  | ⏸ Linux-only | - |
| 6  | cargo audit                     | 0   | ✅ PASS（0/152） | [gate-audit.txt](file:///d:/sqlrustgo/project-main/reports/week-14/gate-audit.txt) / [gate-audit.json](file:///d:/sqlrustgo/project-main/reports/week-14/gate-audit.json) |

### 4.3 测试详情

- **lib tests**：360 passed; 0 failed; 0 ignored
- **auth tests**：22 passed; 0 failed
- **ci tests**：5 passed; 0 failed
- **integration**：8 passed; 0 failed
- **project**：4 passed; 0 failed
- **qps benchmarks**：4 ignored（long-running by default）
- **doctest**：1 ignored

**总计**：399 passed; 0 failed; 5 ignored

### 4.4 发现的问题

| 优先级  | 问题               | 修复建议                              |
| ---- | ---------------- | --------------------------------- |
| 🔴 高  | 5 个文件未通过 `cargo fmt --check` | 运行 `cargo fmt` 后重新提交              |
| 🟡 中  | clippy 2 个 style 警告 | 修复 `src/storage/file_storage.rs:259,262` 的 `collapsible_match` |
| 🟢 低  | tarpaulin 在 Windows 不可用 | 改用 grcov 或迁移到 WSL/CI Linux 跑      |
| 🟢 低  | API 文档未达 100%     | 补充 rustdoc 注释                       |

### 4.5 提交方式

```bash
git checkout -b experiment/week-14-202442020128
mkdir -p reports/week-14
# 放入报告
git add reports/week-14/
git commit -m "experiment: submit week-14 report"
git push origin experiment/week-14-202442020128
```

***

## 五、遇到的问题与解决

### 5.1 cargo build panic on Windows MAX_PATH

**问题**：`cargo build --release` 触发 `Result::unwrap()` on `Err` value: `Os { code: 0, kind: Uncategorized, message: "操作成功完成" }`（GBK 编码，实际是"操作成功完成"），是 serde build script 调用 `Command::output()` 失败。

**原因**：默认 target 目录 `D:\sqlrustgo\project-main\target\release\build\serde-...` 超过 Windows MAX_PATH（260 字符）。

**解决**：
1. `TEMP=C:\tmp`（缩短临时目录）
2. `CARGO_TARGET_DIR=C:\t`（缩短 target 目录到 4 字符）

修完后 `cargo build --release` 顺利通过。

### 5.2 cargo fmt panic 同根问题

**问题**：`cargo fmt --check` 同样 panic 在 `Command::output()`。

**原因**：`cargo fmt` 内部调用 `rustfmt`，路径过长。

**解决**：直接用 `rustfmt --check --edition 2021 <file>` 逐文件检查，记录 5 个 bad 文件。

### 5.3 沙箱 RUSTUP_HOME 漂移

**问题**：第一次跑 `cargo audit --offline` 时报 "rustup could not choose a default toolchain"。

**原因**：之前 `setx RUSTUP_HOME="C:\rustup"` 把错误的全局环境变量写入了 user env。

**解决**：清掉 user env 中的 `RUSTUP_HOME` / `CARGO_HOME`，每次命令前显式 `$env:RUSTUP_HOME = "C:\Users\qiao\.rustup"` 临时设置。

### 5.4 develop/v3.0.0 不存在

**问题**：v5 规范要求 RC 验收时 `git checkout develop/v3.0.0`。

**现状**：项目没有该分支，活跃分支为 `experiment/week-XX-*` 系列。

**降级**：在 `experiment/week-14-202442020128` 上执行门禁作为 RC 验收（v5 规范的"足够近似"）。

***

## 六、实验总结

### 6.1 门禁清单设计的"必须 vs 最好"

**必须（Must-Pass）**：
- cargo build --release
- cargo test --all-features
- cargo audit
- 集成测试

**最好（Should-Pass）**：
- clippy 0 warnings
- cargo fmt --check
- 覆盖率 ≥80%

**待评估（Nice-to-Have）**：
- 性能 QPS
- API 文档 100%

### 6.2 AI 不可替代的发布决策

门禁脚本可以机械化判定"通过/失败"，但下列判断 AI 无法做：

1. **业务风险权衡**：5 个 fmt 失败 vs 2 个 clippy 警告 vs 0 个安全漏洞 —— 哪个更阻塞？
2. **时间窗口决策**：周五晚 9 点发现 1 个低危安全漏洞，是否今天发？还是周一？
3. **用户影响评估**：这个性能回归只影响 1% 用户，是否阻断发布？
4. **跨团队协调**：QA 没回归完成 vs Release Notes 没写完 vs 运维没排期发布窗口 —— 哪个先？

### 6.3 与已有 `scripts/gate/gate.sh` 的关系

项目已有 [`scripts/gate/gate.sh`](file:///d:/sqlrustgo/project-main/scripts/gate/gate.sh) 完整门禁（7 步：质量/覆盖率/安全/文档/安装/分支保护/最终状态）。本次实验的 [`scripts/run_gates.sh`](file:///d:/sqlrustgo/project-main/scripts/run_gates.sh) 按 v5 规范要求**重新设计**，更精简（6 步），聚焦在 CI 可执行的 5 步 + 安全审计。

两者关系：`run_gates.sh` = 快速 CI 门禁，`gate.sh` = 完整发布门禁。

### 6.4 RC 验收的"伪通过"问题

本次执行了 4 步 + 跳过 2 步（tarpaulin + 性能），这暴露了**门禁的诚实性问题**：

- 如果把"跳过"也算"通过"，就是 false positive
- 应该区分 ⏸ 跳过 vs ✅ 通过
- 建议在 `run_gates.sh` 中把 `--skip-coverage` / `--skip-perf` 设计为显式 flag，CI 缺工具时拒绝继续

### 6.5 自我提升

本周完成了"修车技能"的两周闭环：
- week-13：扫描现有问题（cargo audit、clippy、安全审查）
- week-14：建立防护规则（门禁脚本、检查清单）

接下来 week-15+ 应该把门禁脚本**接入 CI**（`.github/workflows/release-gate.yml`），让规则自动跑、失败自动拒合。

***

## 七、AI 工具使用记录

### 7.1 本次 AI 辅助内容

- **清单结构设计**：参考 v5 规范的 6 维度划分
- **脚本设计**：参考 v5 规范 6 步 + Windows 兼容增强（tarpaulin fallback）
- **结果汇总**：从 5 个 gate-*.log 提取数据写入清单
- **报告结构**：复用 week-12/13 报告模板

### 7.2 AI 无法替代的判断

- "必须 vs 最好"门禁分类（基于业务优先级，AI 没有上下文）
- RC 验收降级策略（项目无 develop/v3.0.0，决定在哪个分支验收）
- "跳过"vs"通过"语义区分（门禁诚实性）

### 7.3 AI 提示模板

复用 [week-12 ai-prompt](file:///d:/sqlrustgo/project-main/reports/week-12/ai-security-review-prompt.md) 的结构，无新增。

***

## 八、参考资料

- v5 规范：[reports/week-14-发布门禁与检查清单-v5.md](file:///d:/sqlrustgo/project-main/reports/week-14-发布门禁与检查清单-v5.md)
- [docs/RELEASE_GATES_COMPREHENSIVE.md](file:///d:/sqlrustgo/project-main/docs/RELEASE_GATES_COMPREHENSIVE.md)
- [docs/RC_BRANCH_PROTECTION.md](file:///d:/sqlrustgo/project-main/docs/RC_BRANCH_PROTECTION.md)
- [docs/releases/v1.0.0/](file:///d:/sqlrustgo/project-main/docs/releases/v1.0.0/) 现有 release 目录结构
- [scripts/gate/gate.sh](file:///d:/sqlrustgo/project-main/scripts/gate/gate.sh) 项目已有门禁脚本

***

## 九、评分标准

| 检查项    | 分值   |
| ----- | ---- |
| 门禁检查清单设计 | 20 分 |
| 门禁脚本编写  | 25 分 |
| 门禁执行结果  | 25 分 |
| 问题记录    | 15 分 |
| 实验报告完整  | 15 分 |
| **总分**  | **100 分** |

***

## 十、附录

### 附录 A：门禁脚本 [`scripts/run_gates.sh`](file:///d:/sqlrustgo/project-main/scripts/run_gates.sh)

参见文件本身，共 6 步，含 set -e、Windows tarpaulin fallback。

### 附录 B：门禁执行日志

| 文件 | 用途 |
| -- | -- |
| [gate-build.txt](file:///d:/sqlrustgo/project-main/reports/week-14/gate-build.txt) | cargo build --release |
| [gate-test.txt](file:///d:/sqlrustgo/project-main/reports/week-14/gate-test.txt) | cargo test --all-features |
| [gate-clippy.txt](file:///d:/sqlrustgo/project-main/reports/week-14/gate-clippy.txt) | cargo clippy --all-features |
| [gate-fmt.txt](file:///d:/sqlrustgo/project-main/reports/week-14/gate-fmt.txt) | cargo fmt --check（fallback: rustfmt） |
| [gate-audit.txt](file:///d:/sqlrustgo/project-main/reports/week-14/gate-audit.txt) | cargo audit 文本摘要 |
| [gate-audit.json](file:///d:/sqlrustgo/project-main/reports/week-14/gate-audit.json) | cargo audit --json 完整结果 |

### 附录 C：文件清单

```
docs/releases/v2.6.0/
└── RELEASE_GATE_CHECKLIST.md     # 6 维度清单 + 总结 + 决策

scripts/
└── run_gates.sh                   # 6 步门禁脚本

reports/week-14/
├── week-14-实验报告.md             # 本文档
├── gate-build.txt
├── gate-test.txt
├── gate-clippy.txt
├── gate-fmt.txt
├── gate-audit.txt
└── gate-audit.json
```

### 附录 D：关键命令清单

```bash
# 1. 创建清单
mkdir -p docs/releases/v2.6.0
# 编辑 RELEASE_GATE_CHECKLIST.md

# 2. 创建脚本
# 编辑 scripts/run_gates.sh
chmod +x scripts/run_gates.sh

# 3. 运行门禁（每项）
cargo build --release
cargo test --all-features
cargo clippy --all-features
cargo fmt --check --all
cargo tarpaulin --out Xml --packages parser,executor,storage  # Linux only
cargo audit

# 4. 提交推送
git checkout -b experiment/week-14-202442020128
git add docs/releases/v2.6.0/ scripts/run_gates.sh reports/week-14/
git commit -m "week-14: add release gate checklist and script"
git push origin experiment/week-14-202442020128
```

### 附录 E：未执行的步骤说明

- **步骤 4.1 切换到 develop/v3.0.0**：项目无此分支，降级到 `experiment/week-14-202442020128` 执行验收。
- **步骤 2.5 cargo tarpaulin**：Windows 不可用，跳过；建议改用 grcov 或在 CI Linux 跑。
- **性能门禁**：未跑 QPS 基准（依赖硬件与负载），留待 CI 环境执行。

### 附录 F：commit 与 push 记录

（实验完成后填充）
