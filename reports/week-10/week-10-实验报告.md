# 实验报告

| 项目       | 内容                               |
| -------- | -------------------------------- |
| **实验名称** | Harness治理实战 —— DELETE/UPDATE性能优化 |
| **实验周次** | 第 10 周                           |
| **实验日期** | 2026 年 5 月 16 日                 |
| **学生姓名** | 阳奇                               |
| **学号**   | 202442020128                     |
| **班级**   | 2024级软件工程1班                      |
| **指导教师** | 李莹                               |

***

## 一、实验目的

1. 理解 Harness 治理的三层模型（提示词-上下文-Harness）
2. 能够运行 QPS 基准测试并分析性能瓶颈
3. 能够使用 AI 辅助定位性能问题根因
4. 能够模拟 Gate 检查流程（BP1 → BP2 拦截 → 修复 → 通过）
5. 能够绘制 Harness 治理闭环图

***

## 二、实验环境

### 2.1 硬件环境

| 项目    | 配置                                     |
| ----- | -------------------------------------- |
| 计算机型号 | LENOVO 83DG                            |
| CPU   | Intel(R) Core(TM) i7-14650HX (16核24线程) |
| 内存    | 16GB                                   |
| 硬盘    | C盘: 300GB, D盘: 650GB                   |

### 2.2 软件环境

| 软件   | 版本                                    |
| ---- | ------------------------------------- |
| 操作系统 | Microsoft Windows 11 专业版 (10.0.26200) |
| Rust | 1.96.0                                |
| Git  | 2.45.2.windows.1                      |
| IDE  | Trae IDE                              |
| AI工具 | Claude Code, OpenCode                 |

***

## 三、实验内容与步骤

### 3.1 阶段转变说明

**本周是"Harness治理实战"第1周！**

| 上周（第9周）   | 本周（第10周）          |
| --------- | ----------------- |
| 软件治理与分支策略 | **Harness治理实战**   |
| PR工作流     | **性能优化 + Gate检查** |
| 多AI协同开发   | **AI辅助根因分析**      |

**本周的核心转变**：

- 从"写代码"转向"让代码在约束下走向正确"
- 理解 Harness 不是帮 AI 写代码，而是告诉 AI 哪里还不够好
- 通过客观指标（QPS）而非主观判断来评估代码质量
- 体验完整的治理闭环：Issue → PR → Gate拦截 → 根因分析 → 修复 → 通过

***

### 3.2 步骤0：确认分支环境

#### 0.1 查看当前分支

```bash
git branch --show-current
```

**输出结果**：

```
main
```

#### 0.2 尝试切换到 develop/v3.0.0

```bash
git checkout develop/v3.0.0
```

**输出结果**：

```
error: pathspec 'develop/v3.0.0' did not match any file(s) known to git
```

#### 0.3 查看所有远程分支

```bash
git branch -a
```

**输出结果**：

```
  docs/module-design-week6
  experiment/week-08-202442020128
  feature/week9-lab
* main
  master
  remotes/origin/docs/module-design-week6
  remotes/origin/experiment/week-08-202442020128
  remotes/origin/master
  remotes/qiaob8/docs/module-design-week6
  remotes/qiaob8/experiment/week-08-202442020128
  remotes/qiaob8/feature/week9-lab
  remotes/qiaob8/main
  remotes/qiaob8/master
```

#### 0.4 环境适配说明

由于远程仓库没有 `develop/v3.0.0` 分支，本次实验基于当前 `main` 分支（v1.0.0）进行。为保证实验完整性，自行创建了 QPS 基准测试文件 `tests/qps_benchmark_test.rs`。

#### ✅ 检查点0：分支确认完成

| 项目   | 结果                    |
| ---- | --------------------- |
| 目标分支 | develop/v3.0.0（不存在）   |
| 实际分支 | main（v1.0.0）          |
| 适配措施 | 基于 main 分支创建 QPS 基准测试 |

***

### 3.3 步骤1：运行 QPS 基准测试 —— 亲眼看"慢"

#### 1.1 创建 QPS 基准测试文件

创建 `tests/qps_benchmark_test.rs`，包含 DELETE/UPDATE/INSERT/SELECT 四种操作的基准测试：

```rust
use sqlrustgo::{parse, ExecutionEngine};
use std::time::Instant;

const BENCHMARK_ITERATIONS: usize = 1000;

fn create_engine() -> ExecutionEngine {
    ExecutionEngine::new()
}

fn setup_tables(engine: &mut ExecutionEngine) {
    let _ = engine.execute(parse("CREATE TABLE users (id INTEGER, name TEXT, age INTEGER)").unwrap());
}
```

#### 1.2 运行 DELETE 基准测试

```bash
cargo test --test qps_benchmark_test test_qps_delete -- --ignored --nocapture
```

**输出结果**：

```
DELETE QPS: 1000 queries in 6.81s (146.86 qps)
```

**记录实际 QPS 值**：`146.86 QPS`

#### 1.3 运行 UPDATE 基准测试

```bash
cargo test --test qps_benchmark_test test_qps_update -- --ignored --nocapture
```

**输出结果**：

```
UPDATE QPS: 1000 queries in 11.99s (83.38 qps)
```

**记录实际 QPS 值**：`83.38 QPS`

#### 1.4 运行 INSERT 基准测试

```bash
cargo test --test qps_benchmark_test test_qps_insert -- --ignored --nocapture
```

**输出结果**：

```
INSERT QPS: 1000 queries in 11.29s (88.58 qps)
```

**记录实际 QPS 值**：`88.58 QPS`

#### 1.5 运行 SELECT 基准测试

```bash
cargo test --test qps_benchmark_test test_qps_simple_select -- --ignored --nocapture
```

**输出结果**：

```
SELECT QPS: 1000 queries in 0.32s (3097.08 qps)
```

**记录实际 QPS 值**：`3097.08 QPS`

#### 1.6 填写性能基线表

| 操作     | QPS     | 与 E-09 地板 (10,000) 比较 |
| ------ | ------- | --------------------- |
| DELETE | 146.86  | ❌ 不达标                 |
| UPDATE | 83.38  | ❌ 不达标                 |
| INSERT | 88.58   | —                     |
| SELECT | 3097.08 | —                     |

#### ✅ 检查点1：QPS 基准测试完成

**关键发现**：

- DELETE 只有 **146.86 QPS**，删除 10,000 行需要约 **68 秒**
- UPDATE 只有 **83.38 QPS**，更新 10,000 行需要约 **120 秒**
- SELECT 达到 **3,097 QPS**，是 DELETE 的 **21 倍**
- 这与实验文档中 v2.9.0 的性能危机（DELETE 206 QPS）处于同一数量级

***

### 3.4 步骤2：分析根因 —— AI + 人工协作

#### 2.1 阅读基准测试代码

测试代码逻辑：

1. 创建 `ExecutionEngine`
2. 创建 `users` 表
3. 插入 1000 行数据
4. 逐条执行 DELETE/UPDATE（测试逐条操作性能）
5. 计算 QPS

#### 2.2 用 AI 辅助分析

**向 AI 提问**：

```
SQLRustGo 的 DELETE 操作只有 ~146 QPS，而 SELECT 有 ~3,097 QPS。
为什么 DELETE 比 SELECT 慢 21 倍？可能的原因有哪些？

请从以下角度分析：
1. DELETE 操作在数据库内部需要做哪些额外工作（vs SELECT）？
2. 当前实现中可能有哪些性能瓶颈？
3. 给出 2-3 个可能的优化方向。
```

**AI 分析结果**：

1. **DELETE 的额外工作**：
   - SELECT 只需读取内存数据并过滤返回
   - DELETE 需要修改内存数据结构 + 持久化到磁盘
   - 当前实现中，DELETE 后调用了 `persist_table()` 进行全表序列化
2. **性能瓶颈定位**：
   - 通过阅读 `src/executor/mod.rs`（第484-534行）和 `src/storage/file_storage.rs`（第155-173行）发现：
     - 每次 DELETE/UPDATE/INSERT 后都调用 `self.storage.persist_table(&stmt.table)?`
     - `persist_table()` 调用 `save_table()`，使用 `serde_json::to_string_pretty()` 将整个表序列化为 JSON
     - 然后写入文件系统
   - **关键发现**：每执行一条 DELETE，就把整个表重新序列化并写盘一次
3. **优化方向**：
   - **批量提交**：不在每条 SQL 后写盘，而是批量或事务结束时写盘
   - **WAL（预写日志）**：只追加写日志，而不是重写整个表文件
   - **异步/延迟写盘**：利用 BufferPool，定期刷盘而不是实时刷盘

#### 2.3 记录根因分析

| 可能原因          | 为什么影响 DELETE/UPDATE                                             | 优化方向                              |
| ------------- | --------------------------------------------------------------- | --------------------------------- |
| 每次操作后同步全表写盘   | `persist_table()` 每次都将整个表序列化为 pretty JSON 并写入文件，O(n) 复杂度随表增大而恶化 | 引入 WAL 机制，改为追加写日志；或延迟批量刷盘         |
| JSON 文本序列化开销大 | `serde_json::to_string_pretty()` 生成格式化 JSON，比二进制格式慢数倍           | 使用二进制格式（如 bincode）或分页存储           |
| 无事务/批量处理机制    | 每条 SQL 独立解析、执行、持久化，无法摊平写盘开销                                     | 支持显式事务 BEGIN/COMMIT，在 COMMIT 时才刷盘 |

#### ✅ 检查点2：根因分析完成

***

### 3.5 步骤3：模拟 Gate 检查 —— 理解"约束"的作用

#### 3.1 场景设定

假设你是开发者，刚刚"优化"了 DELETE 的实现，提交了 PR #500。现在 Harness Gate 系统启动。

#### 3.2 模拟 BP1 静态检查

```bash
echo "=== BP1 静态检查 ==="
cargo build --all-features 2>&1 | tail -1
echo "---"
cargo fmt --check --all 2>&1 | tail -1
echo "---"
cargo clippy --all-features -- -D warnings 2>&1 | tail -1
```

**实际执行结果**：

- `cargo build`：由于 Rust 工具链 build script panic 问题，无法正常运行
- `cargo fmt`：同上，环境限制
- `cargo clippy`：同上，环境限制
- **但** **`cargo test`** **可以正常编译和运行**，说明代码本身编译通过

| BP1 检查项 | 结果                 |
| ------- | ------------------ |
| 编译      | ✅（cargo test 成功运行） |
| 格式      | ⚠️ 环境限制            |
| Clippy  | ⚠️ 环境限制            |

#### 3.3 模拟 BP2 行为检查

```bash
echo "=== BP2 行为检查 ==="
cargo test --test qps_benchmark_test -- --ignored --nocapture 2>&1 | grep "QPS:"
```

**实际执行结果**：

| BP2 检查项    | QPS 结果     | 是否通过 (≥10,000?) |
| ---------- | ---------- | --------------- |
| DELETE QPS | **146.86** | ❌ **FAIL**      |
| UPDATE QPS | **83.38** | ❌ **FAIL**      |

#### 3.4 理解"被 Gate 拦截"意味着什么

```
DELETE QPS = 146 < 10,000
  → BP2 Gate FAIL
  → PR #500 被 Blocked
  → 开发者收到通知："DELETE QPS 未达到 E-09 地板 (10,000)"
  → 必须修复后重新提交

这不是"你的代码不好"，
而是"客观指标告诉你：这个版本还不能发布"。

类比：体检报告说你血压偏高 —— 不是医生在骂你，是数据在提醒你。
```

**如果 Gate 不存在会怎样？**

- DELETE 146 QPS 的版本会被合并到主分支
- 用户在删除大数据表时会经历数十秒的等待
- 性能问题可能在生产环境暴露后才被发现，修复成本更高

#### ✅ 检查点3：Gate 模拟完成

***

### 3.6 步骤4：绘制 Harness 治理闭环图

#### 4.1 闭环结构

```
┌─────────────────────────────────────────────────────────────┐
│              Harness 治理闭环（DELETE/UPDATE 优化案例）        │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ① Issue: "DELETE QPS 需从 146 → 10,000+"                   │
│     "UPDATE QPS 需从 83 → 10,000+"                          │
│          │                                                  │
│          ▼                                                  │
│  ② AI/人写代码 → PR #500                                    │
│     当前实现：每次操作后调用 persist_table() 全表写盘         │
│          │                                                  │
│          ▼                                                  │
│  ③ Gate 检查                                                │
│     BP1 ✅ 编译通过                                          │
│     BP2 ❌ DELETE 146 QPS < 10,000（FAIL）                  │
│     BP2 ❌ UPDATE 83 QPS < 10,000（FAIL）                   │
│          │                                                  │
│          ▼                                                  │
│  ④ 分析根因 → 设计优化方案                                  │
│     根因：每次 DELETE/UPDATE 后全表 JSON 序列化 + 同步写盘   │
│     方案：                                                  │
│       · 引入 WAL（预写日志），追加写代替全表重写              │
│       · 支持事务批量提交，COMMIT 时才刷盘                     │
│       · BufferPool 延迟写盘策略                              │
│          │                                                  │
│          ▼                                                  │
│  ⑤ 重新 PR → Gate 通过 → 合并                              │
│     优化后：DELETE QPS 从 146 → 目标 10,000+                │
│             UPDATE QPS 从 83 → 目标 10,000+                 │
│          │                                                  │
│          ▼                                                  │
│  ⑥ 知识沉淀                                                 │
│     · 更新基准数据（QPS 从 146 → 10,000+）                   │
│     · 记录 Pattern 到 GBrain："不要每条 SQL 后全表写盘"      │
│     · 评估：Gate 阈值 10,000 是否合理？是。                  │
│          │                                                  │
│          └──→ 回到 ①（下一轮优化，如 INSERT 优化）           │
│                                                             │
│  【如果 Gate 不存在】                                        │
│     → 146 QPS 的版本会静默进入主分支                         │
│     → 用户生产环境删除 1万行数据需 68 秒                     │
│     → 问题可能在客户投诉后才暴露，修复成本 ×10               │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

#### 4.2 每一步的实际数据标注

| 步骤        | 实际做了什么                            | 关键数据                           |
| --------- | --------------------------------- | ------------------------------ |
| ① Issue   | 发现 DELETE/UPDATE 性能远低于可用标准        | DELETE 146 QPS, UPDATE 83 QPS |
| ② PR #500 | 当前代码每次操作后全表写盘                     | —                              |
| ③ Gate    | BP1 编译通过，BP2 QPS 检查 FAIL          | 146 < 10,000 ❌                 |
| ④ 根因分析    | 定位到 `persist_table()` 全表 JSON 序列化 | O(n) 复杂度                       |
| ⑤ 修复      | 引入 WAL + 批量提交 + 延迟写盘              | 目标 ≥10,000                     |
| ⑥ 知识沉淀    | 更新基准，记录 Pattern                   | 146 → 10,000+                  |

#### ✅ 检查点4：治理闭环图完成

***

### 3.7 步骤5：三层模型反思

#### 5.1 对照 DELETE/UPDATE 案例

**提示词层**：

- 如果只说"优化 DELETE 性能"而不给出量化目标（≥10,000 QPS），AI 可能只做微小调整
- → 治理改进：PR 模板必须写清楚量化目标

**上下文层**：

- AI 能看到当前 DELETE 的实现代码
- AI 看不到：全表 JSON 序列化的开销、每次写盘对性能的影响、真实生产环境（10万行表）下的表现
- → 治理改进：向 AI 提供基准测试数据和性能 Profile

**Harness 层**：

- Gate 不管"AI 怎么实现的"，只管"QPS 达标了吗？"
- BP2: DELETE QPS 实测 → 146 < 10,000 → ❌ FAIL
- → Harness = 把"主观判断"变成"客观指标"

#### 5.2 填写反思表

| 层级        | 本次实验中你注意到什么问题？                | 下次如何改进？                          |
| --------- | ----------------------------- | -------------------------------- |
| 提示词层      | 只说"优化性能"没有量化目标，AI 无法判断是否达标    | PR 模板必须写清楚量化目标；向 AI 提问时明确性能门槛    |
| 上下文层      | AI 看不到存储引擎的写盘策略和性能开销          | 向 AI 提供基准测试数据、性能 Profile、存储引擎架构图 |
| Harness 层 | Gate 用客观指标（QPS）拦截了主观上"能运行"的代码 | 坚持数据驱动，即使开发者认为"代码没问题"，数据不达标就不能发布 |

#### ✅ 检查点5：三层分析完成

***

## 四、实验结果

### 4.1 QPS 基准测试数据

| 操作     | QPS        | 与 E-09 地板 (10,000) 比较 |
| ------ | ---------- | --------------------- |
| DELETE | **146.86** | ❌ 不达标 (差距 68x)        |
| UPDATE | **83.38**  | ❌ 不达标 (差距 120x)       |
| INSERT | 88.58      | —                     |
| SELECT | 3097.08    | —                     |

### 4.2 根因分析结果

| 根因           | 影响                                           | 优化方向               |
| ------------ | -------------------------------------------- | ------------------ |
| 每次操作后同步全表写盘  | DELETE/UPDATE 后调用 `persist_table()`，O(n) 复杂度 | 引入 WAL，延迟批量刷盘      |
| JSON 文本序列化开销 | `serde_json::to_string_pretty()` 全表序列化       | 使用二进制格式或分页存储       |
| 无事务批量机制      | 每条 SQL 独立持久化                                 | 支持 BEGIN/COMMIT 事务 |

### 4.3 Gate 模拟结果

| 检查项            | 结果     | 说明              |
| -------------- | ------ | --------------- |
| BP1 编译         | ✅      | cargo test 成功运行 |
| BP1 格式         | ⚠️     | 环境限制            |
| BP1 Clippy     | ⚠️     | 环境限制            |
| BP2 DELETE QPS | ❌ FAIL | 146.86 < 10,000 |
| BP2 UPDATE QPS | ❌ FAIL | 83.38 < 10,000 |

### 4.4 完成情况

| 任务               | 状态  | 说明                                             |
| ---------------- | --- | ---------------------------------------------- |
| QPS 基准测试（4组）     | ✅完成 | DELETE 146, UPDATE 83, INSERT 88, SELECT 3097 |
| 根因分析（AI+人工）      | ✅完成 | 定位到 persist\_table() 全表写盘问题                    |
| Gate 模拟（BP1+BP2） | ✅完成 | BP2 DELETE/UPDATE 均 FAIL                       |
| Harness 治理闭环图    | ✅完成 | 包含6步闭环和"如果Gate不存在"分析                           |
| 三层模型反思           | ✅完成 | 提示词层/上下文层/Harness层                             |

***

## 五、实验心得与总结

### 5.1 Harness 治理的价值

通过本次实验，我深刻理解了 Harness 治理的核心价值：

1. **把主观判断变成客观指标**：Gate 不管"代码看起来好不好"，只管"QPS 达标了吗"
2. **在合并前拦截问题**：146 QPS 的版本在 PR 阶段就被拦截，不会进入生产环境
3. **强制根因分析**：被 Gate 拦截后，必须深入分析才能找到真正的优化方向
4. **知识沉淀**：每次拦截都记录 Pattern，避免团队重复踩坑

### 5.2 性能优化的收获

**根因定位方法**：

- 先跑基准测试，拿到量化数据
- 对比不同操作的性能差异（SELECT 3097 vs DELETE 146）
- 顺着"为什么 DELETE 要做更多工作"追问
- 阅读存储层代码，找到 `persist_table()` 这个瓶颈点

**优化方向**：

- 不要每条 SQL 后全表写盘
- 引入 WAL（预写日志）做追加写
- 支持事务批量提交
- BufferPool 延迟刷盘

### 5.3 三层模型的思考

**提示词层**：

- 优势：AI 能快速理解需求并生成代码
- 局限：没有量化目标，AI 不知道"够不够好"
- 改进：PR 模板必须包含性能门槛

**上下文层**：

- 优势：AI 能看到代码实现细节
- 局限：看不到运行时性能特征和生产环境场景
- 改进：向 AI 提供 Profile 数据和架构上下文

**Harness 层**：

- 优势：用客观指标做最终判定
- 价值：即使前两层都"看起来对了"，Harness 说不行就是不行
- 本质：**约束驱动迭代**

***

## 六、思考题

### 6.1 如果没有 Gate 系统，DELETE 只有 146 QPS 的版本会被发现吗？什么时候才会被发现？

**回答**：

- **可能不会**。开发者自己测试时可能只测了 10 条数据，感觉"挺快的"
- 只有到了生产环境，用户删除大量数据时才会发现（客户投诉）
- 或者等到系统压力测试时才暴露，但此时修复成本已经很高
- Gate 的作用：**在代码合并前就拦截问题**，把问题发现成本从"生产事故"降到"PR 阶段"

### 6.2 AI 在这个优化过程中能做什么、不能做什么？

**回答**：

- AI **能**：
  - 分析代码瓶颈（如指出 `persist_table()` 的调用）
  - 提出优化方案（如 WAL、批量提交）
  - 写优化代码
- AI **不能**：
  - 知道 QPS 门槛是 10,000（需要 Harness 提供约束）
  - 知道用户真实使用场景（需要上下文补充）
  - 判断"这个性能是否可接受"（需要客观指标）
- → AI 需要 Harness 提供**量化约束**才能产出"达标"的代码

### 6.3 为什么说"Harness 不是帮 AI 写代码，而是告诉 AI 哪里还不够好"？

**回答**：

- 本实验中，AI 可以写出"能运行"的 DELETE 代码（功能正确）
- 但 AI 不知道这个 DELETE 在 1000 次循环下只有 146 QPS
- Gate（BP2）运行后告诉 AI/开发者："QPS 不达标，需要优化"
- AI 根据这个反馈，才能针对性地优化存储层写盘策略
- → Harness 是**反馈环**，让 AI 在约束下迭代改进
- 没有 Harness，AI 可能永远不知道"146 QPS 不够好"

***

## 七、评分标准对照

| 检查项               | 分值  | 完成情况  |
| ----------------- | --- | ----- |
| QPS 基准测试数据（4组）    | 15分 | ✅ 已完成 |
| 根因分析（AI辅助 + 人工判断） | 20分 | ✅ 已完成 |
| Gate 模拟结果         | 20分 | ✅ 已完成 |
| Harness 治理闭环图     | 25分 | ✅ 已完成 |
| 三层模型反思            | 10分 | ✅ 已完成 |
| 实验报告完整            | 10分 | ✅ 已完成 |

***

## 八、附录

### 8.1 QPS 基准测试代码

**文件**：`tests/qps_benchmark_test.rs`

**核心逻辑**：

```rust
fn test_qps_delete() {
    let mut engine = create_engine();
    setup_tables(&mut engine);

    // 先插入 1000 行
    for i in 0..BENCHMARK_ITERATIONS {
        let _ = engine.execute(
            parse(&format!("INSERT INTO users VALUES ({}, 'bench_{}', {})", i, i, 30)).unwrap()
        );
    }

    // 然后逐条 DELETE（测试逐条删除性能）
    let start = Instant::now();
    for i in 0..BENCHMARK_ITERATIONS {
        let _ = engine.execute(
            parse(&format!("DELETE FROM users WHERE id = {}", i)).unwrap()
        );
    }
    let duration = start.elapsed();
    let qps = BENCHMARK_ITERATIONS as f64 / duration.as_secs_f64();
    println!("DELETE QPS: {} queries in {:.2}s ({:.2} qps)",
             BENCHMARK_ITERATIONS, duration.as_secs_f64(), qps);
}
```

### 8.2 关键代码路径

**DELETE 执行路径**：

- `src/executor/mod.rs:484-534` — `execute_delete()` 函数
- 第527行：`self.storage.persist_table(&stmt.table)?` — 每次 DELETE 后全表写盘

**UPDATE 执行路径**：

- `src/executor/mod.rs:417-481` — `execute_update()` 函数
- 第474行：`self.storage.persist_table(&stmt.table)?` — 每次 UPDATE 后全表写盘

**存储层写盘逻辑**：

- `src/storage/file_storage.rs:155-173` — `save_table()` 函数
- 第166行：`serde_json::to_string_pretty(&stored)` — 全表 JSON 序列化

### 8.3 实验运行记录

```bash
# 运行 DELETE 基准测试
cargo test --test qps_benchmark_test test_qps_delete -- --ignored --nocapture
# 输出：DELETE QPS: 1000 queries in 6.81s (146.86 qps)

# 运行 UPDATE 基准测试
cargo test --test qps_benchmark_test test_qps_update -- --ignored --nocapture
# 输出：UPDATE QPS: 1000 queries in 11.99s (83.38 qps)

# 运行 INSERT 基准测试
cargo test --test qps_benchmark_test test_qps_insert -- --ignored --nocapture
# 输出：INSERT QPS: 1000 queries in 11.29s (88.58 qps)

# 运行 SELECT 基准测试
cargo test --test qps_benchmark_test test_qps_simple_select -- --ignored --nocapture
# 输出：SELECT QPS: 1000 queries in 0.32s (3097.08 qps)
```

***

| 指导教师 | \_\_\_\_\_\_\_\_\_\_\_\_\_\_\_\_\_\_ | 实验成绩   | \_\_\_\_\_\_\_\_\_\_\_\_\_\_\_\_\_\_ |
| ---- | ------------------------------------ | ------ | ------------------------------------ |
| 批改日期 | \_\_\_\_\_\_\_\_\_\_\_\_\_\_\_\_\_\_ | <br /> | <br />                               |

