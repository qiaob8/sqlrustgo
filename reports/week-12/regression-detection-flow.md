# 回归检测流程图（Regression Detection Flow）

> 实验周次：第 12 周
> 实验类型：回归检测与自优化闭环

---

## 一、ASCII 完整流程图

```
  ┌─────────────────────┐
  │ 1. 触发场景          │
  │                     │
  │ ① 开发者 git push   │
  │ ② 每日定时任务        │
  │ ③ 手动检查           │
  └──────────┬──────────┘
             │
             ▼
  ┌─────────────────────┐
  │ 2. CI 调度           │
  │  Gitea Actions      │
  │  → 触发 ci.yml      │
  │  → Nomad worker     │
  └──────────┬──────────┘
             │
             ▼
  ┌─────────────────────┐
  │ 3. BP1 静态检查       │
  │  build / fmt /     │
  │  clippy            │
  └──────┬──────────────┘
         │
    ┌────┴────┐
    ▼         ▼
  ✅         ❌ ─→ 终止 + 通知
    │
    ▼
  ┌─────────────────────┐
  │ 4. BP2 集成 + 行为   │
  │  cargo test         │
  │  + qps_benchmark    │
  └──────┬──────────────┘
         │
    ┌────┴────┐
    ▼         ▼
  ✅         ❌ ─→ 终止 + 通知
    │
    ▼
  ┌─────────────────────┐
  │ 5. 提取 QPS 数据     │
  │  解析 qps_results.log│
  │  → current.json     │
  └──────┬──────────────┘
         │
         ▼
  ┌─────────────────────┐
  │ 6. 读取 baseline    │
  │  data/qps_baseline │
  │  .json             │
  └──────┬──────────────┘
         │
         ▼
  ┌─────────────────────┐
  │ 7. check_regression │
  │     .sh             │
  │  ────────────────   │
  │  计算 Δ%             │
  │  判定 PASS/WARN/FAIL│
  └──────┬──────────────┘
         │
    ┌────┼────┬──────────┐
    ▼    ▼    ▼          │
  PASS  WARN FAIL        │
    │    │    │          │
    │    │    ▼          │
    │    │  ┌──────────────────┐
    │    │  │ 8a. FAIL 处置     │
    │    │  │                  │
    │    │  │ • PR ⛔ Blocked  │
    │    │  │ • 邮件通知        │
    │    │  │ • Slack @dev    │
    │    │  │ • 创建 Issue    │
    │    │  └────────┬─────────┘
    │    │           │
    │    │           ▼
    │    │     开发者修复
    │    │           │
    │    │           ▼
    │    │     重新 push ──→ 回到 ②
    │    │
    │    ▼
    │  ┌──────────────────┐
    │  │ 8b. WARN 处置     │
    │  │                  │
    │  │ • 允许合并        │
    │  │ • 评论要求说明     │
    │  │ • 标记为可疑 PR   │
    │  └────────┬─────────┘
    │           │
    │           ▼
    │       合并
    │
    ▼
  ┌──────────────────┐
  │ 8c. PASS 处置     │
  │                  │
  │ • ✅ 允许合并    │
  │ • CI 徽章绿      │
  │ • 自动归档数据   │
  └──────────────────┘
```

---

## 二、Mermaid 流程图

```mermaid
flowchart TD
    Trigger[触发场景<br/>① push / ② 定时 / ③ 手动]
    Trigger --> CI[CI 调度<br/>Gitea Actions]
    CI --> BP1{CI BP1 静态<br/>build / fmt / clippy}
    BP1 -->|❌ FAIL| Notify1[通知开发者<br/>PR Blocked]
    BP1 -->|✅ PASS| BP2{CI BP2 集成<br/>cargo test}
    BP2 -->|❌ FAIL| Notify1
    BP2 -->|✅ PASS| QPS[跑 QPS 基准<br/>生成 current.json]
    QPS --> Baseline[读取 baseline<br/>data/qps_baseline.json]
    Baseline --> Check{check_regression.sh<br/>计算 Δ% 并判定}
    Check -->|Δ ≤ 5% PASS| MergeP[✅ 允许合并]
    Check -->|5% < Δ ≤ 20% WARN| MergeW[允许合并<br/>+ 评论说明]
    Check -->|Δ > 20% FAIL| Block[⛔ PR Blocked<br/>+ 邮件/Slack/Issue]
    Block -.修复后重 push.-> CI
    MergeW --> MergeP

    style Trigger fill:#2196F3,color:#fff
    style MergeP fill:#4CAF50,color:#fff
    style Block fill:#F44336,color:#fff
    style Notify1 fill:#F44336,color:#fff
    style MergeW fill:#FF9800,color:#fff
    style Check fill:#9C27B0,color:#fff
```

---

## 三、PDCA 自优化闭环时序图

```mermaid
sequenceDiagram
    autonumber
    participant Dev as 开发者
    participant CI as CI
    participant Reg as 回归检测
    participant KB as 知识库
    participant Notif as 通知服务

    Note over Dev,Notif: 第 N 轮 PDCA

    Dev->>CI: git push
    CI->>CI: BP1 / BP2 检查
    CI->>Reg: 跑 QPS 基准
    Reg->>Reg: 计算 Δ%
    Reg-->>CI: PASS / WARN / FAIL

    alt FAIL
        Reg-->>Dev: PR ⛔ Blocked
        Reg->>Notif: 邮件 / Slack @dev
        Notif->>Dev: 收到通知
        Dev->>Dev: 分析根因
        Dev->>Dev: 修复
        Dev->>CI: 重新 push
    else PASS
        CI-->>Dev: ✅ 允许合并
        Dev->>KB: 沉淀根因 + 解决方案
        Dev->>Reg: --save-baseline
        Reg->>Reg: 更新 baseline
    end

    Note over Dev,Notif: 第 N+1 轮 PDCA 开始
```

---

## 四、回归检测判定矩阵

| Δ% 范围     | 判定    | 退出码 | 处置                     | 阈值来源                    |
| --------- | ----- | --- | ---------------------- | ----------------------- |
| Δ < 0%    | PASS↑ | 0   | ✅ 允许合并 + 自动归档         | 性能提升（好事）                |
| 0% ≤ Δ ≤ 5% | PASS  | 0   | ✅ 允许合并                | 噪声范围（不可控）               |
| 5% < Δ ≤ 20% | WARN  | 2   | ⚠️ 允许合并 + 强制评论说明      | 可疑退化（可能是真实问题）           |
| Δ > 20%   | FAIL  | 1   | ⛔ PR Blocked + 三通道通知    | 严重退化（必须修复）              |

---

## 五、与 PDCA 阶段的对应

| 阶段    | 本流程中的位置          | 关键产物                           |
| ----- | ---------------- | ------------------------------ |
| **Plan** | 触发场景（push / 定时）   | 目标（如 DELETE QPS ≥ 10K）         |
| **Do**   | CI BP1 / BP2     | 代码改动、PR、CI 报告                 |
| **Check**| check_regression.sh | PASS / WARN / FAIL + Δ%        |
| **Act**  | 通知 / 沉淀 / 更新基线   | 知识库条目、新 baseline.json、下一轮目标    |

---

## 六、与现有 Gate 脚本的集成

```
scripts/gate/
├── gate.sh                  ← 总入口
│   ├── check_coverage.sh    ← 覆盖率 ≥ 80%
│   ├── check_security.sh    ← 安全扫描
│   ├── check_docs.sh        ← 文档检查
│   └── check_regression.sh  ← 🆕 本次新增：性能回归
```

集成到 `gate.sh` 的方法：

```bash
# 在 gate.sh 末尾追加
echo "=== Regression Check ==="
scripts/gate/check_regression.sh --skip-run
```

---

## 七、为什么 --skip-run 不能用于 Gate 决策？

| 用途       | --skip-run | 完整跑（默认） |
| -------- | ---------- | -------- |
| 调试脚本逻辑   | ✅ 适用       | ❌ 浪费时间  |
| 复现 Gate 决策 | ✅ 适用       | ❌ 浪费时间  |
| **实际 Gate 决策** | ❌ **绝对不可** | ✅ **唯一可用** |
| 归档基线     | ✅ 适用       | ✅ 都可    |

**原因**：`--skip-run` 永远返回 `current.json = baseline.json`，Δ% 永远 = 0%，永远 PASS。
这等于"不检测"，违背了"通过 Gate 必须有真凭实据"的原则。

---

*最后更新: 2026-05-30*
