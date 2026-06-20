# Gate 联动流程图（CI/CD 自动化执行引擎）

> 实验周次：第 11 周
> 实验类型：CI/CD与Agent编排

---

## 一、ASCII 全景图（8 步完整链路）

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│                  Gate 联动流程（push → 反馈）                                │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ① 开发者 push 代码                                                          │
│     git push origin develop/v3.0.0                                          │
│     ┌──────────────────┐                                                   │
│     │ developer/本地仓库 │                                                   │
│     └────────┬─────────┘                                                   │
│              │ (1) push event                                               │
│              ▼                                                              │
│  ② Gitea 接收 webhook                                                       │
│     ┌──────────────────┐                                                   │
│     │  Gitea Server     │                                                   │
│     │  POST /webhook    │                                                   │
│     │  → 触发 ci.yml    │                                                   │
│     └────────┬─────────┘                                                   │
│              │ (2) enqueue Job                                              │
│              ▼                                                              │
│  ③ Nomad 调度 Job                                                           │
│     ┌──────────────────┐                                                   │
│     │  Nomad Cluster   │                                                    │
│     │  worker 节点拉取   │                                                    │
│     │  ubuntu-latest    │                                                   │
│     └────────┬─────────┘                                                   │
│              │ (3) run jobs                                                 │
│              ▼                                                              │
│  ④ BP1 静态检查 (bp1-static)                                                │
│     ┌──────────────────────────────────────┐                               │
│     │  cargo build --all-features          │                               │
│     │  cargo fmt  --check --all            │                               │
│     │  cargo clippy --all-features -D warn │                               │
│     └────────┬─────────────────────────────┘                               │
│              │                                                              │
│       ┌──────┴──────┐                                                       │
│       ▼             ▼                                                       │
│    ✅ PASS       ❌ FAIL ──────► (A) 中断回环                                │
│       │                                                                  │
│       │ (4) needs 满足                                                     │
│       ▼                                                                  │
│  ⑤ BP2 集成 / 行为检查 (bp2-integration)                                   │
│     ┌──────────────────────────────────────┐                               │
│     │  cargo test --all-features            │                               │
│     │  cargo test --test qps_benchmark_test│                               │
│     └────────┬─────────────────────────────┘                               │
│              │                                                              │
│       ┌──────┴──────┐                                                       │
│       ▼             ▼                                                       │
│    ✅ PASS       ❌ FAIL ──────► (A) 中断回环                                │
│       │                                                                  │
│       │ (5) 提交质量达标                                                     │
│       ▼                                                                  │
│  ⑥ BP3 风险检查（多 Agent 编排）                                          │
│     ┌──────────────┬──────────────┬──────────────┐                          │
│     │   explore    │  librarian   │    oracle    │                          │
│     │  静态代码扫描  │  查外部文档    │   风险评级    │                          │
│     │  影响面报告    │  最佳实践清单  │  PASS/CONDI  │                          │
│     │              │              │  /BLOCK      │                          │
│     └──────┬───────┴──────┬───────┴──────┬───────┘                          │
│            │              │              │                                  │
│            └──────────────┼──────────────┘                                  │
│                           ▼                                                  │
│                  ┌──────────────────┐                                        │
│                  │  Synthesizer     │                                        │
│                  │  合成统一报告       │                                        │
│                  └────────┬─────────┘                                        │
│                           │                                                  │
│                ┌──────────┼──────────┐                                       │
│                ▼          ▼          ▼                                       │
│              PASS     CONDITION   BLOCK ──────► (A) 中断回环                  │
│                │                                                                  │
│                │ (6) 允许合并                                                      │
│                ▼                                                                  │
│  ⑦ 结果上报 PR 评论                                                              │
│     ┌──────────────────┐                                                   │
│     │  Gitea Bot        │                                                   │
│     │  写回 PR 评论       │                                                   │
│     │  + CI 状态徽章     │                                                   │
│     └────────┬─────────┘                                                   │
│              │ (7) commit status event                                      │
│              ▼                                                              │
│  ⑧ 开发者收到通知                                                            │
│     ┌──────────┬──────────┬──────────┐                                      │
│     │   邮件    │  飞书    │  Slack   │                                      │
│     │ @dev@..  │ Webhook  │ Webhook  │                                      │
│     └──────────┴──────────┴──────────┘                                      │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘


   (A) 中断回环（任一步失败都走这里）
   ┌─────────────────────────────────────────┐
   │  Nomad 标记 Job = failed                │
   │       │                                  │
   │       ▼                                  │
   │  Gitea 接收 status 事件                  │
   │       │                                  │
   │       ├─► PR 评论标记 ⛔ Blocked          │
   │       ├─► 邮件发件人 noreply@.gitea.io   │
   │       └─► 飞书/Slack 推送 @developer      │
   │                                          │
   │  开发者修复 → 重新 push → 回到 ②          │
   └─────────────────────────────────────────┘
```

---

## 二、Mermaid 流程图（GitHub/Gitea 自动渲染）

```mermaid
flowchart TD
    Start([① 开发者 git push]) --> Webhook[② Gitea 接收 webhook<br/>触发 ci.yml]
    Webhook --> Nomad[③ Nomad 调度 Job<br/>worker 拉取任务]

    Nomad --> BP1{④ BP1 静态检查<br/>build / fmt / clippy}

    BP1 -->|✅ PASS| BP2
    BP1 -->|❌ FAIL| FailA[中断：标记 Job=failed]

    BP2{⑤ BP2 集成检查<br/>cargo test / qps_benchmark}
    BP2 -->|✅ PASS| BP3
    BP2 -->|❌ FAIL| FailA

    subgraph BP3_Block [⑥ BP3 多 Agent 编排]
        direction LR
        Explore[explore<br/>静态扫描] --> Synth[Synthesizer<br/>合成报告]
        Librarian[librarian<br/>查文档] --> Synth
        Oracle[oracle<br/>风险评级] --> Synth
    end

    BP3 --> BP3_Block
    BP3_Block --> OracleDecision{oracle 决策}

    OracleDecision -->|PASS| PRComment
    OracleDecision -->|CONDITIONAL| PRComment
    OracleDecision -->|BLOCK| FailA

    PRComment[⑦ Gitea Bot 上报 PR<br/>写评论 + 状态徽章]
    PRComment --> Notify[⑧ 多通道通知<br/>邮件 / 飞书 / Slack]

    FailA -.->|开发者修复后重 push| Webhook

    Notify --> End([回环完成])

    style Start fill:#4CAF50,color:#fff
    style End fill:#4CAF50,color:#fff
    style FailA fill:#F44336,color:#fff
    style BP1 fill:#2196F3,color:#fff
    style BP2 fill:#2196F3,color:#fff
    style BP3_Block fill:#FF9800,color:#fff
    style Notify fill:#9C27B0,color:#fff
```

---

## 三、时序图（Sequence Diagram）

```mermaid
sequenceDiagram
    autonumber
    participant Dev as 开发者
    participant Git as Gitea
    participant Nomad as Nomad
    participant CI as CI Runner
    participant Agent as Agent Service
    participant Notif as 通知服务

    Dev->>Git: git push origin develop/v3.0.0
    Git->>Git: 接收 push 事件
    Git->>Nomad: 入队 Job
    Nomad->>CI: 调度到 worker
    CI->>CI: BP1 静态检查

    alt BP1 失败
        CI-->>Git: 状态 = failed
        Git->>Notif: 通知开发者
    else BP1 通过
        CI->>CI: BP2 集成 + QPS
        alt BP2 失败
            CI-->>Git: 状态 = failed
            Git->>Notif: 通知开发者
        else BP2 通过
            CI->>Agent: 触发多 Agent 编排
            par 并行执行
                Agent->>Agent: explore 影响面
            and
                Agent->>Agent: librarian 最佳实践
            and
                Agent->>Agent: oracle 风险评级
            end
            Agent->>CI: 合成报告 + 决策
            alt BLOCK
                CI-->>Git: 状态 = failed
                Git->>Notif: 通知开发者
            else PASS / CONDITIONAL
                CI->>Git: 上报 PR 评论 + 状态
                Git->>Notif: 通知开发者
            end
        end
    end
```

---

## 四、失败分支示意

```
任何一步 ❌ FAIL
   │
   ▼
┌──────────────────────┐
│ Nomad 标记 Job 状态   │
│ = failed              │
└──────────┬───────────┘
           │
           ▼
┌──────────────────────┐
│ Gitea 接收 status 事件 │
└──────────┬───────────┘
           │
   ┌───────┼───────┐
   ▼       ▼       ▼
PR 评论   邮件    飞书/Slack
标记 ⛔   发件人   推送
Blocked  noreply  @developer
         @gitea
```

---

## 五、与第10周 Gate 模拟的对照

| 第10周                              | 第11周                                              |
| --------------------------------- | ------------------------------------------------ |
| Gate = 本地脚本模拟（`echo "=== BP1 ==="`） | Gate = CI 工作流（`.gitea/workflows/ci.yml`）      |
| 一次跑通 BP1 + BP2                 | 每次 push / PR 自动跑                                |
| 手动汇总日志                          | Gitea 自动汇总到 PR 评论 + 通知                        |
| BP3 = 概念                         | BP3 = 多 Agent 编排（explore / librarian / oracle） |
| 反馈延迟：开发完才知                     | 反馈即时：每次提交即知                                    |

**关键认知**：CI/CD 不是替代 Gate，而是把 Gate **常态化、自动化**——以前是"开发完手动跑一次"，现在是"每次提交自动跑"。

---

## 六、节点说明

| 节点              | 角色                    | 关键动作                                          |
| --------------- | --------------------- | --------------------------------------------- |
| ① push          | 开发者 / 本地仓库           | `git push origin develop/v3.0.0`              |
| ② webhook       | Gitea Server          | 接收 push 事件，触发 Actions workflow             |
| ③ Nomad         | 集群调度器                | 把 Job 调度到空闲 worker                           |
| ④ BP1           | 静态检查 Job              | 编译 / 格式 / Clippy 三件套                         |
| ⑤ BP2           | 集成 / 行为 Job            | 单元测试 + QPS 性能                                 |
| ⑥ BP3           | 风险 / Agent 编排 Job      | explore + librarian + oracle 并行              |
| ⑦ PR Comment    | Gitea Bot             | 汇总 BP1/BP2/BP3 结果，写回 PR 评论                  |
| ⑧ 通知           | 邮件 / 飞书 / Slack       | 同步推送结果给开发者                                   |
| (A) 中断回环        | 任何失败都触发              | 标记 failed → 通知 → 开发者修复 → 重新 push → 回到 ②    |

---

## 七、对应本仓库的具体实现

| 链路节点 | 本仓库的落地文件 / 命令                                                                  |
| -------- | ---------------------------------------------------------------------------------- |
| ① push    | `git push origin develop/v3.0.0`（当前分支 `experiment/week-10-202442020128` 暂未推送） |
| ② webhook | Gitea 接收事件                                                                     |
| ③ Job    | `.gitea/workflows/ci.yml` 中的 `bp1-static` / `bp2-integration` 两个 job              |
| ④ BP1    | `cargo build --all-features` / `cargo fmt --check --all` / `cargo clippy ... -D warnings` |
| ⑤ BP2    | `cargo test --all-features` / `cargo test --test qps_benchmark_test -- --ignored --nocapture` |
| ⑥ BP3    | 设计中的多 Agent 编排（explore / librarian / oracle），后续可加 `bp3-agent-review` job      |
| ⑦ 上报   | Gitea Bot 自动写回 PR 评论                                                            |
| ⑧ 通知   | Gitea → Webhook → 邮件 / 飞书 / Slack                                              |

---

*最后更新: 2026-05-23*
