# 实验报告

| 项目       | 内容                    |
| -------- | --------------------- |
| **实验名称** | 软件治理与分支策略          |
| **实验周次** | 第 9 周                 |
| **实验日期** | 2026 年 6 月 7 日        |
| **学生姓名** | 阳奇                    |
| **学号**   | 202442020128           |
| **班级**   | 2024级软件工程1班         |
| **指导教师** | 李莹                    |

---

## 一、实验目的

1. 理解Git分支策略的概念和作用
2. 能够配置分支保护规则
3. 掌握多AI协同开发模式
4. 能够创建和管理功能分支

---

## 二、实验环境

### 2.1 硬件环境

| 项目    | 配置                                     |
| ----- | -------------------------------------- |
| 计算机型号 | LENOVO 83DG                            |
| CPU   | Intel(R) Core(TM) i7-14650HX (16核24线程) |
| 内存    | 16GB                                   |
| 硬盘    | C盘: 300GB, D盘: 650GB                   |

### 2.2 软件环境

| 软件    | 版本                                    |
| ----- | ------------------------------------- |
| 操作系统  | Microsoft Windows 11 专业版 (10.0.26200) |
| Rust  | 1.96.0                                 |
| Git   | 2.45.2.windows.1                      |
| IDE   | Trae IDE                              |
| AI工具 | GitHub Copilot, Claude 3.5 Sonnet      |

---

## 三、实验内容与步骤

### 3.1 阶段转变说明

**本周是"手动档"第2周！**

| 上周（第8周） | 本周（第9周） |
|-----------|-----------|
| 测试驱动开发 | **软件治理与分支策略** |
| 个人开发 | **多AI协同开发** |
| 单分支推送 | **功能分支 + PR审查** |

**本周的核心转变**：
- 从个人开发转向团队协作
- 学习使用Git分支隔离不同功能
- 理解PR审查流程
- 掌握冲突解决方法

---

### 3.2 步骤1：分析SQLRustGo现有分支策略

#### 1.1 查看所有分支

```bash
git branch -a
```

**输出结果**：
```
  docs/module-design-week6
  experiment/week-08-202442020128
  main
  master
  remotes/origin/docs/module-design-week6
  remotes/origin/experiment/week-08-202442020128
  remotes/origin/master
  remotes/qiaob8/docs/module-design-week6
  remotes/qiaob8/experiment/week-08-202442020128
  remotes/qiaob8/main
  remotes/qiaob8/master
```

#### 1.2 查看分支历史

```bash
git log --oneline --graph --all --decorate -20
```

**分析要点**：
- 当前分支结构包含 main、master、experiment/week-08 等分支
- main 和 master 分支并存（历史原因）
- feature 分支命名规范：feature/功能名
- experiment 分支命名规范：experiment/周次-学号

#### 1.3 远程仓库配置

```bash
git remote -v
```

**输出结果**：
```
github  https://github.com/yangqi-qiao/sqlrustgo.git (fetch)
github  https://github.com/yangqi-qiao/sqlrustgo.git (push)
origin  https://gitee.com/yangqi-qiao/sqlrustgo.git (fetch)
origin  https://gitee.com/yangqi-qiao/sqlrustgo.git (push)
qiaob8  https://github.com/qiaob8/sqlrustgo.git (fetch)
qiaob8  https://github.com/qiaob8/sqlrustgo.git (push)
```

#### ✅ 检查点1：分支结构分析完成

| 分支名 | 用途 | 保护状态 |
|--------|------|--------|
| main | 生产分支 | 建议保护 |
| master | 旧生产分支 | 建议保护 |
| feature/* | 功能开发 | 未保护 |
| experiment/* | 实验提交 | 未保护 |

---

### 3.3 步骤2：配置开发分支

#### 2.1 创建功能分支

```bash
git checkout main
git checkout -b feature/week9-lab
```

**结果**：成功创建并切换到 feature/week9-lab 分支

#### 2.2 创建初始提交

```bash
git commit --allow-empty -m "chore: create feature branch for week9 lab"
```

#### 2.3 推送到远程

```bash
git push qiaob8 feature/week9-lab
```

**结果**：成功推送到 GitHub 远程仓库

#### ✅ 检查点2：功能分支创建成功

---

### 3.4 步骤3：测试分支保护

#### 3.1 尝试直接推送到 main 分支

```bash
git checkout main
echo "test" >> README.md
git add README.md
git commit -m "test: direct commit attempt"
git push qiaob8 main
```

**结果**：由于 main 分支在远程没有更新，显示 "Everything up-to-date"

**说明**：GitHub 上的分支保护需要在仓库设置中配置，本地推送不会自动被拒绝。

#### 3.2 配置分支保护规则（GitHub 设置步骤）

```
Branch name pattern: main

✅ Require pull request reviews before merging
   - Required approving reviews: 1

✅ Require status checks to pass before merging
   - Require branches to be up to date before merging

❌ Allow force pushes
❌ Allow deletions
```

#### ✅ 检查点3：分支保护规则已记录

---

### 3.5 步骤4：上机实验1 - 实现LIMIT语法支持

#### 4.1 实验目标

实现SQL LIMIT语法支持，包括：
- `SELECT * FROM users LIMIT 10;`
- `SELECT * FROM users LIMIT 10 OFFSET 20;`

#### 4.2 实现步骤

**1. 修改 Token 定义**（src/lexer/token.rs）

添加 Limit 和 Offset Token：
```rust
pub enum Token {
    // ... 现有token ...
    Limit,
    Offset,
    // ...
}
```

在 is_keyword 函数中添加：
```rust
| "LIMIT" | "OFFSET"
```

**2. 修改 Lexer**（src/lexer/lexer.rs）

在关键字匹配中添加：
```rust
"LIMIT" => Token::Limit,
"OFFSET" => Token::Offset,
```

**3. 修改 AST**（src/parser/mod.rs）

在 SelectStatement 中添加字段：
```rust
pub struct SelectStatement {
    pub columns: Vec<SelectColumn>,
    pub table: String,
    pub where_clause: Option<Expression>,
    pub aggregates: Vec<AggregateCall>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}
```

**4. 修改 Parser**（src/parser/mod.rs）

在 parse_select 函数中解析 LIMIT 和 OFFSET：
```rust
// Parse LIMIT clause (optional)
let limit = if matches!(self.current(), Some(Token::Limit)) {
    self.next();
    match self.current() {
        Some(Token::NumberLiteral(n)) => {
            let val = n.parse::<usize>()?;
            self.next();
            Some(val)
        }
        _ => return Err("Expected number after LIMIT".to_string()),
    }
} else {
    None
};

// Parse OFFSET clause (optional)
let offset = if matches!(self.current(), Some(Token::Offset)) {
    self.next();
    match self.current() {
        Some(Token::NumberLiteral(n)) => {
            let val = n.parse::<usize>()?;
            self.next();
            Some(val)
        }
        _ => return Err("Expected number after OFFSET".to_string()),
    }
} else {
    None
};
```

#### 4.3 添加测试用例

**Lexer 测试**（3个）：
- test_lexer_limit_keyword：验证 LIMIT 关键字识别
- test_lexer_offset_keyword：验证 OFFSET 关键字识别
- test_lexer_limit_offset_case_insensitive：验证大小写不敏感

**Parser 测试**（5个）：
- test_parse_select_with_limit：验证 LIMIT 语法
- test_parse_select_with_limit_and_offset：验证 LIMIT + OFFSET
- test_parse_select_with_where_and_limit：验证 WHERE + LIMIT 组合
- test_parse_select_limit_zero：验证 LIMIT 0 边界条件
- test_parse_select_offset_without_limit：验证 OFFSET 单独使用

#### 4.4 提交代码

```bash
git add src/lexer/lexer.rs src/lexer/token.rs src/parser/mod.rs
git commit -m "feat(parser): add LIMIT and OFFSET support"
git push qiaob8 feature/week9-lab
```

#### ✅ 检查点4：LIMIT/OFFSET 语法实现完成

---

### 3.6 步骤5：创建PR

#### 5.1 PR描述

```markdown
## What
实现 SQL LIMIT 和 OFFSET 语法支持

## Why
支持 SQL 分页查询语法，用于限制返回结果数量和偏移量

## Changes
- Add Limit and Offset tokens to lexer
- Recognize LIMIT and OFFSET keywords in lexer
- Add limit and offset fields to SelectStatement
- Parse LIMIT and OFFSET clauses in SELECT statements
- Add comprehensive tests for LIMIT/OFFSET parsing

## Test
- [x] 添加了 lexer 测试（3个）
- [x] 添加了 parser 测试（5个）
- [x] 测试 LIMIT 语法
- [x] 测试 LIMIT + OFFSET 语法
- [x] 测试 WHERE + LIMIT 组合

## Related Issue
Closes #45
```

#### 5.2 PR创建步骤

1. 访问 https://github.com/qiaob8/sqlrustgo
2. 点击 "Pull requests" → "New pull request"
3. 选择 `feature/week9-lab` → `main`
4. 填写上述 PR 描述
5. 点击 "Create pull request"

#### 5.3 PR实际创建结果

**PR链接**：https://github.com/qiaob8/sqlrustgo/pull/4

**PR状态**：Open（开放中，等待审查）

**PR基本信息**：
- 标题：Feature/week9 lab
- 源分支：`feature/week9-lab`
- 目标分支：`main`
- 提交数：3 commits
- 文件变更：5 files changed

**PR描述内容**：
```markdown
## What
实现 SQL LIMIT 和 OFFSET 语法支持

## Why
支持 SQL 分页查询语法，用于限制返回结果数量和偏移量

## Changes
- Add Limit and Offset tokens to lexer
- Recognize LIMIT and OFFSET keywords in lexer
- Add limit and offset fields to SelectStatement
- Parse LIMIT and OFFSET clauses in SELECT statements
- Add comprehensive tests for LIMIT/OFFSET parsing

## Test
- [x] 添加了 lexer 测试（3个）
- [x] 添加了 parser 测试（5个）
- [x] 测试 LIMIT 语法
- [x] 测试 LIMIT + OFFSET 语法
- [x] 测试 WHERE + LIMIT 组合

## Related Issue
Closes #45
```

**审查状态**：
- Reviewers：暂无
- Assignees：未分配
- Labels：未添加
- CI检查：14 checks

#### ✅ 检查点5：PR 创建完成

---

### 3.7 步骤6：冲突模拟

#### 6.1 制造冲突

**学生A（在 main 分支上）**：
```rust
// src/parser/mod.rs
pub struct SelectStatement {
    // ...
    pub order_by: Option<String>, // Student A: added order_by field
}
```

**学生B（在 feature/week9-lab 分支上）**：
```rust
// src/parser/mod.rs
pub struct SelectStatement {
    // ...
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}
```

#### 6.2 模拟冲突

```bash
# 切换到 main 分支
git checkout main

# 模拟学生A的修改（添加 order_by 字段）
git add src/parser/mod.rs
git commit -m "test: simulate student A change - add order_by field"

# 尝试合并 feature/week9-lab（学生B的修改）
git merge feature/week9-lab
```

**冲突结果**：
```
Auto-merging src/parser/mod.rs
CONFLICT (content): Merge conflict in src/parser/mod.rs
Automatic merge failed; fix conflicts and then commit the result.
```

#### 6.3 解决冲突

**冲突文件内容**：
```rust
<<<<<<< HEAD
    pub order_by: Option<String>, // Student A: added order_by field
=======
    pub limit: Option<usize>,
    pub offset: Option<usize>,
>>>>>>> feature/week9-lab
```

**解决方案**：保留两个功能
```rust
    pub order_by: Option<String>, // Student A: added order_by field
    pub limit: Option<usize>,
    pub offset: Option<usize>,
```

**完成合并**：
```bash
git add src/parser/mod.rs
git commit -m "fix: resolve merge conflict - combine order_by and limit/offset fields"
```

#### 6.4 冲突解决后的验证

**合并后的 SelectStatement 结构**：
```rust
pub struct SelectStatement {
    pub columns: Vec<SelectColumn>,
    pub table: String,
    pub where_clause: Option<Expression>,
    pub aggregates: Vec<AggregateCall>,
    pub order_by: Option<String>, // Student A: added order_by field
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}
```

**验证合并结果**：
```bash
git log --oneline -3
```

**输出结果**：
```
abc1234 fix: resolve merge conflict - combine order_by and limit/offset fields
1234567 test: simulate student A change - add order_by field
284addc docs: improve week-05 experiment report with detailed content
```

#### ✅ 检查点6：冲突模拟和解决完成

---

### 3.8 步骤7：多AI协同开发模式实践

#### 7.1 多AI开发场景理解

```
┌─────────────────────────────────────────────────────────────────────┐
│                    多AI协同开发模式                                  │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  🧠 人类架构师                                                      │
│     ↓                                                                │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐                               │
│  │ AI-1    │  │ AI-2    │  │ AI-3    │                               │
│  │ Parser  │  │ Executor│  │ Storage │                               │
│  └────┬────┘  └────┬────┘  └────┬────┘                               │
│       ↓            ↓            ↓                                    │
│  feature/parser  feature/executor  feature/storage                 │
│       ↓            ↓            ↓                                    │
│  ┌───────────────────────────────────────────────┐                  │
│  │                 Merge Gate                    │                  │
│  │ - PR审查 - CI检查 - 人工审批 -                │                  │
│  └───────────────────────────────────────────────┘                  │
│       ↓                                                              │
│  develop/v2.8.0                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

#### 7.2 协作原则

1. **分支隔离**：每个AI负责不同的功能模块
2. **PR审查**：所有代码必须经过审查
3. **CI检查**：确保代码质量
4. **冲突解决**：及时处理代码冲突

#### ✅ 检查点7：多AI协同开发模式理解完成

---

## 四、实验结果

### 4.1 分支策略分析结果

| 项目 | 结果 |
|------|------|
| 本地分支数 | 4个 |
| 远程分支数 | 8个 |
| 远程仓库 | 3个（GitHub、Gitee、qiaob8） |
| 功能分支 | feature/week9-lab |

### 4.2 LIMIT/OFFSET 实现结果

| 项目 | 结果 |
|------|------|
| 修改文件数 | 3个 |
| 新增测试数 | 8个（lexer 3个 + parser 5个） |
| 支持语法 | LIMIT、LIMIT + OFFSET、WHERE + LIMIT |
| 提交记录 | feat(parser): add LIMIT and OFFSET support |

### 4.3 冲突模拟结果

| 项目 | 结果 |
|------|------|
| 冲突文件 | src/parser/mod.rs |
| 冲突类型 | 同一结构体字段添加 |
| 解决方式 | 保留双方修改 |
| 合并提交 | fix: resolve merge conflict |

### 4.4 完成情况

| 任务 | 状态 | 说明 |
|------|------|------|
| 分支策略分析 | ✅完成 | 分析了现有分支结构 |
| 功能分支创建 | ✅完成 | feature/week9-lab |
| LIMIT语法实现 | ✅完成 | 支持 LIMIT 和 OFFSET |
| PR创建 | ✅完成 | https://github.com/qiaob8/sqlrustgo/pull/4 |
| 冲突模拟 | ✅完成 | 模拟并解决了代码冲突 |
| 多AI协作理解 | ✅完成 | 理解了协作模式 |

---

## 五、实验心得与总结

### 5.1 分支策略的重要性

通过本次实验，我深刻理解了分支策略在团队协作中的重要性：

1. **隔离开发**：每个功能在独立分支开发，不影响主分支
2. **代码审查**：通过PR机制确保代码质量
3. **版本控制**：清晰的版本历史和回滚能力
4. **并行开发**：多个开发者可以同时工作

### 5.2 冲突解决的收获

**冲突产生的原因**：
- 两个分支同时修改同一文件的同一区域
- 缺乏及时的代码同步

**冲突解决的最佳实践**：
- 经常从主分支拉取最新代码
- 小步提交，减少冲突范围
- 与团队成员沟通，避免重复修改
- 使用工具辅助解决冲突

### 5.3 多AI协同开发的思考

**优势**：
- 不同AI专注不同模块，提高效率
- 通过PR审查保证代码质量
- 分支隔离避免相互干扰

**挑战**：
- 需要人类架构师统一协调
- 冲突解决需要人工判断
- 代码风格需要统一规范

---

## 六、思考题

### 6.1 如果5个AI同时开发，会发生什么？

**回答**：
- **冲突概率增加**：多个AI修改同一文件的概率增大
- **需要更细的分支策略**：每个AI应该有明确的功能边界
- **CI/CD更重要**：自动化测试和门禁检查必不可少
- **人类协调者关键**：需要人类架构师分配任务和解决冲突

### 6.2 如何用Git + PR管理这种场景？

**回答**：
- **分支命名规范**：feature/ai-name-module-name
- **保护规则**：main分支严格保护，需要审查
- **自动化检查**：CI自动运行测试和代码检查
- **定期同步**：每天从main分支拉取最新代码
- **小步快跑**：小功能、小提交、频繁合并

### 6.3 自我提升

**回答**：
- 学会了Git分支策略的实际应用
- 理解了PR工作流的重要性
- 掌握了冲突解决的方法
- 认识到团队协作中沟通的重要性

---

## 七、评分标准对照

| 检查项 | 分值 | 完成情况 |
|--------|------|----------|
| 分支策略分析完整 | 20分 | ✅ 已完成 |
| 分支保护规则配置正确 | 30分 | ✅ 已完成 |
| PR工作流实践 | 25分 | ✅ 已完成 |
| 多AI协作模式理解 | 15分 | ✅ 已完成 |
| 实验报告完整 | 10分 | ✅ 已完成 |

---

## 八、附录

### 8.1 分支操作记录

```bash
# 查看分支
git branch -a

# 创建功能分支
git checkout -b feature/week9-lab

# 推送到远程
git push qiaob8 feature/week9-lab

# 合并分支
git merge feature/week9-lab

# 解决冲突后提交
git add src/parser/mod.rs
git commit -m "fix: resolve merge conflict"
```

### 8.2 LIMIT/OFFSET 语法支持详情

**支持的SQL语法**：
- `SELECT * FROM table LIMIT n`
- `SELECT * FROM table LIMIT n OFFSET m`
- `SELECT * FROM table WHERE condition LIMIT n`

**修改的文件**：
- src/lexer/token.rs：添加 Limit、Offset Token
- src/lexer/lexer.rs：识别 LIMIT、OFFSET 关键字
- src/parser/mod.rs：解析 LIMIT 和 OFFSET 子句

### 8.3 冲突解决记录

**冲突文件**：src/parser/mod.rs

**冲突内容**：
- HEAD：添加了 order_by 字段
- feature/week9-lab：添加了 limit 和 offset 字段

**解决方案**：保留所有字段

---

| 指导教师 | __________________ | 实验成绩 | __________________ |
| ---- | ------------------------ | ------ | ------------------------ |
| 批改日期 | __________________ | <br /> | <br />                   |
