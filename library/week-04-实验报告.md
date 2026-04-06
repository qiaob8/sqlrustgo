# 实验报告

| 项目       | 内容              |
| -------- | --------------- |
| **实验名称** | UML建模与OOA-OOD实践 |
| **实验周次** | 第 4 周           |
| **实验日期** | 2026 年 4 月 6 日  |
| **学生姓名** | 阳奇              |
| **学号**   | 202442020128    |
| **班级**   | 2024级软件工程班1     |
| **指导教师** | 李莹              |

***

## 一、实验目的

1. 掌握用例图绘制方法
2. 掌握活动图绘制方法（流程视角）
3. 理解架构设计的重要性
4. 掌握顺序图绘制方法（对象视角）
5. 掌握类图的更新方法
6. 掌握状态图绘制方法
7. 理解OOA→OOD→OOP的转换过程

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

| 软件    | 版本                                    |
| ----- | ------------------------------------- |
| 操作系统  | Microsoft Windows 11 专业版 (10.0.26200) |
| Rust  | 1.93.1 (01f6ddf75 2026-02-11)         |
| Git   | 2.45.2.windows.1                      |
| IDE   | Trae IDE                              |
| UML工具 | Mermaid                               |

***

## 三、实验内容与步骤

### 3.1 用例图

**步骤1：识别参与者**

| 参与者   | 说明                 |
| ----- | ------------------ |
| 学生    | 借书、还书、查询、预约、查看历史   |
| 教师    | 借书、还书、查询、预约、查看历史   |
| 图书管理员 | 管理图书、查看所有借阅记录、管理用户 |

**步骤2：识别用例**

| 用例       | 说明              |
| -------- | --------------- |
| 登录       | 用户身份验证          |
| 查询图书     | 搜索和查看图书信息       |
| 借书       | 借阅图书            |
| 还书       | 归还图书            |
| 预约图书     | 预约已被借出的图书       |
| 查看借阅历史   | 查看个人借阅记录        |
| 管理图书     | 图书管理员添加/修改/删除图书 |
| 查看所有借阅记录 | 管理员查看所有借阅情况     |
| 计算罚款     | 计算逾期罚款          |
| 管理用户     | 管理员添加/冻结/解冻用户   |

**步骤3：用例图**

```mermaid
%%{init: {'theme': 'forest', 'themeVariables': { 'primaryColor': '#4A90D9', 'primaryTextColor': '#fff', 'primaryBorderColor': '#2C5F8D', 'secondaryColor': '#95C11F', 'fontSize': '14px'}}}%%
graph TD
    subgraph "外部参与者"
        Student["学生"]
        Teacher["教师"]
        Librarian["图书管理员"]
    end

    subgraph "系统用例"
        Login["登录"]
        QueryBook["查询图书"]
        BorrowBook["借书"]
        ReturnBook["还书"]
        ReserveBook["预约图书"]
        ViewHistory["查看借阅历史"]
        ManageBook["管理图书"]
        ViewAllRecords["查看所有借阅记录"]
        CalculateFine["计算罚款"]
        ManageUser["管理用户"]
        RenewBook["续借图书"]
    end

    %% 学生与用例的关系
    Student --> Login
    Student --> QueryBook
    Student --> BorrowBook
    Student --> ReturnBook
    Student --> ReserveBook
    Student --> ViewHistory
    Student --> RenewBook

    %% 教师与用例的关系
    Teacher --> Login
    Teacher --> QueryBook
    Teacher --> BorrowBook
    Teacher --> ReturnBook
    Teacher --> ReserveBook
    Teacher --> ViewHistory
    Teacher --> RenewBook

    %% 图书管理员与用例的关系
    Librarian --> Login
    Librarian --> ManageBook
    Librarian --> ViewAllRecords
    Librarian --> ManageUser
    Librarian --> CalculateFine

    %% 用例之间的关系
    BorrowBook ..> QueryBook : <<include>>
    ReturnBook ..> QueryBook : <<include>>
    ReturnBook ..> CalculateFine : <<include>>
    ReserveBook -.-> BorrowBook : <<extend>>
    RenewBook -.-> BorrowBook : <<extend>>
```

**用例图说明**：
- **参与者**：学生、教师、图书管理员
- **核心用例**：
  - 登录：所有用户的身份验证
  - 查询图书：搜索和查看图书信息
  - 借书：借阅图书
  - 还书：归还图书并计算罚款
  - 预约图书：预约已被借出的图书
  - 续借图书：延长图书借阅期限
  - 查看借阅历史：查看个人借阅记录
  - 管理图书：添加/修改/删除图书
  - 查看所有借阅记录：管理员查看所有借阅情况
  - 管理用户：添加/冻结/解冻用户

**关系说明**：
- <<include>>：表示包含关系，如借书和还书都需要查询图书
- <<extend>>：表示扩展关系，如预约和续借是对借书功能的扩展

***

### 3.2 活动图

**步骤1：借书活动图**

```mermaid
%%{init: {'theme': 'base'}}%%
flowchart TB
    subgraph 用户域
        A1["用户登录系统"]
        A2["查询目标图书"]
        A3["选择要借的图书副本"]
        A6["确认借书"]
        A9["收到借书结果"]
    end

    subgraph 业务域
        B1["验证用户身份"]
        B2{"用户有效?"}
        B3["检查借阅上限"]
        B4{"未达上限?"}
        B5["查询图书副本状态"]
        B6{"图书存在?"}
        B7{"图书可借?"}
        B8["检查预约权限"]
        B9{"有预约且本人?"}
        B10["更新图书状态为已借"]
        B11["创建借阅记录"]
        B12["返回借书结果"]
    end

    A1 --> B1
    B1 --> B2
    B2 -->|否| B12
    B2 -->|是| B3
    B3 --> B4
    B4 -->|否| B12
    B4 -->|是| B5
    B5 --> B6
    B6 -->|否| B12
    B6 -->|是| B7
    B7 -->|不可借| B8
    B7 -->|可借| B10
    B8 --> B9
    B9 -->|否| B12
    B9 -->|是| B10
    B10 --> B11
    B11 --> B12
    B12 --> A6
    A6 --> A9
```

**步骤2：还书活动图**

```mermaid
%%{init: {'theme': 'base'}}%%
flowchart TB
    subgraph 用户域
        A1["提交还书请求"]
        A2["提供借阅凭证"]
        A3["确认还书"]
        A4["收到还书结果"]
    end

    subgraph 业务域
        B1["验证借阅记录"]
        B2{"记录存在?"}
        B3["计算罚款"]
        B4["更新图书状态为可借"]
        B5["创建还书记录"]
        B6["返回还书结果"]
    end

    A1 --> B1
    B1 --> B2
    B2 -->|否| B6
    B2 -->|是| B3
    B3 --> B4
    B4 --> B5
    B5 --> B6
    B6 --> A3
    A3 --> A4
```

***

### 3.3 架构设计

**步骤1：三层架构设计**

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        三层架构（Three-Tier Architecture）                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   表现层 (Controller)    业务层 (Service)    数据层 (Repository)           │
│   ═════════════════     ══════════════     ══════════════                 │
│                                                                             │
│   ┌─────────────────┐    ┌─────────────────┐  ┌─────────────────┐        │
│   │ BorrowController │    │  BorrowService  │  │ UserRepository  │        │
│   │ ReturnController │    │  ReturnService  │  │ BookRepository  │        │
│   │                  │    │  FineService    │  │ BorrowRepository│        │
│   │ - requestBorrow  │    │                 │  │                 │        │
│   │ - requestReturn  │    │ - 借阅业务逻辑   │  │ - findById     │        │
│   │ - 参数校验      │    │ - 还书业务逻辑   │  │ - save         │        │
│   │ - 返回响应      │    │ - 罚款计算逻辑   │  │ - findByCopyId  │        │
│   └─────────────────┘    └─────────────────┘  └─────────────────┘        │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**步骤2：架构设计原则**

| 原则         | 说明                            |
| ---------- | ----------------------------- |
| 单一职责 (SRP) | 每个类只做一件事                      |
| 依赖倒置 (DIP) | Service依赖Repository接口，不依赖具体实现 |
| 开闭原则 (OCP) | 对扩展开放，对修改关闭                   |

***

### 3.4 顺序图

**步骤1：还书顺序图**

```mermaid
%%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#4A90D9', 'primaryTextColor': '#fff', 'primaryBorderColor': '#2C5F8D'}}}%%
sequenceDiagram
    participant U as User (学生/教师)
    participant RC as ReturnController
    participant RS as ReturnService
    participant UR as UserRepository
    participant BR as BorrowRepository
    participant BS as BookService
    participant FS as FineService

    Note over U,FS: 还书流程开始

    U->>RC: requestReturn(userId, copyId)
    activate RC

    RC->>UR: getUserById(userId)
    activate UR
    UR-->>RC: User
    deactivate UR

    alt 用户不存在
        RC-->>U: 返回错误: 用户无效
    else 用户有效
        RC->>BR: findByCopyId(copyId)
        activate BR
        BR-->>RC: BorrowRecord
        deactivate BR

        alt 借阅记录不存在
            RC-->>U: 返回错误: 借阅记录不存在
        else 借阅记录存在
            RC->>FS: calculateFine(recordId)
            activate FS
            FS->>BR: getBorrowDate(recordId)
            BR-->>FS: borrowDate, dueDate
            FS-->>RC: fineAmount
            deactivate FS

            RC->>BS: updateStatus(copyId, Available)
            activate BS
            BS-->>RC: updateResult
            deactivate BS

            RC->>BR: createReturnRecord(recordId, returnDate, fineAmount)
            activate BR
            BR-->>RC: ReturnRecord
            deactivate BR

            RC-->>U: 返回成功: 还书成功，罚款: ¥{fineAmount}
        end
    end

    deactivate RC
    Note over U,FS: 还书流程结束
```

***

### 3.5 类图更新

**步骤1：完整类图**

```mermaid
%%{init: {'theme': 'base'}}%%
classDiagram
    %% 实体类
    class User {
        -id: Long
        -name: String
        -type: UserType
        -maxBorrowLimit: Int
        -status: UserStatus
        +canBorrow(): Boolean
    }

    class BookCopy {
        -copyId: Long
        -bookId: Long
        -status: CopyStatus
        +isAvailable(): Boolean
    }

    class BorrowRecord {
        -recordId: Long
        -userId: Long
        -copyId: Long
        -borrowDate: DateTime
        -dueDate: DateTime
        -returnDate: DateTime
        -status: RecordStatus
        -fineAmount: BigDecimal
    }

    class Reservation {
        -reservationId: Long
        -userId: Long
        -copyId: Long
        -reservationDate: DateTime
    }

    %% 枚举类
    class UserType {
        <<enumeration>>
        STUDENT
        TEACHER
        LIBRARIAN
    }

    class CopyStatus {
        <<enumeration>>
        AVAILABLE
        BORROWED
        RESERVED
        LOST
    }

    class RecordStatus {
        <<enumeration>>
        BORROWING
        RETURNED
        OVERDUE
    }

    %% 服务类
    class BookService {
        +getBookCopyById(copyId): BookCopy
        +confirmBorrow(copyId): Boolean
        +updateStatus(copyId, status): Boolean
    }

    class BorrowService {
        +requestBorrow(userId, copyId): Result
    }

    class ReturnService {
        +requestReturn(userId, copyId): Result
    }

    class FineService {
        +calculateFine(recordId): BigDecimal
        +isOverdue(borrowDate, returnDate): Boolean
    }

    class UserService {
        +getUserById(userId): User
    }

    %% 仓储类
    class UserRepository {
        +findById(id): User
    }

    class BorrowRepository {
        +save(record): BorrowRecord
        +findByCopyId(copyId): BorrowRecord
        +createReturnRecord(record): BorrowRecord
    }

    class BookRepository {
        +findById(id): Book
        +findCopyById(copyId): BookCopy
    }

    %% 控制类
    class BorrowController {
        +requestBorrow(request): Response
    }

    class ReturnController {
        +requestReturn(request): Response
    }

    %% 关系
    User "1" -- "*" BorrowRecord
    BookCopy "1" -- "*" BorrowRecord
    User "1" -- "*" Reservation
    BookCopy "1" -- "*" Reservation

    BorrowController ..> BookService
    BorrowController ..> BorrowService
    BorrowController ..> UserService
    ReturnController ..> ReturnService
    ReturnController ..> UserService
    ReturnController ..> FineService

    BookService --> BookRepository
    BorrowService --> BorrowRepository
    ReturnService --> BorrowRepository
    ReturnService --> BookService
    ReturnService --> FineService
    UserService --> UserRepository
```

***

### 3.6 状态图

**步骤1：BorrowRecord状态图**

```mermaid
stateDiagram-v2
    [*] --> BORROWING: 借书
    BORROWING --> RETURNED: 还书
    BORROWING --> OVERDUE: 超期
    OVERDUE --> RETURNED: 还书
    RETURNED --> [*]
    OVERDUE --> [*]: 特殊处理
```

***

## 四、实验结果

### 4.1 完成情况

| 任务    | 完成情况 | 说明               |
| ----- | ---- | ---------------- |
| 用例图   | ✅ 完成 | 包含所有参与者和用例，关系正确  |
| 借书活动图 | ✅ 完成 | 流程完整，分支清晰        |
| 还书活动图 | ✅ 完成 | 包含计算罚款逻辑，流程完整    |
| 还书顺序图 | ✅ 完成 | 对象清晰，消息完整，包含罚款计算 |
| 类图更新  | ✅ 完成 | 方法完整，关系正确，层次分明   |
| 状态图   | ✅ 完成 | 状态准确，转换合理        |
| 架构设计  | ✅ 完成 | 三层架构清晰，职责划分合理    |

### 4.2 设计说明

本次实验按照OOA→OOD的正确顺序进行：

1. **用例图**：识别了系统的参与者和功能需求
2. **活动图**：分析了借书和还书的业务流程
3. **架构设计**：采用三层架构，明确了各层职责
4. **顺序图**：展示了对象之间的交互过程
5. **类图**：设计了系统的静态结构
6. **状态图**：描述了借阅记录的状态变化

设计中应用了SOLID原则：

- 单一职责：每个类只负责一个功能
- 依赖倒置：Service依赖Repository接口
- 开闭原则：通过接口实现扩展

***

## 五、实验心得与总结

### 5.1 实验收获

1. **UML建模能力**：掌握了用例图、活动图、顺序图、类图和状态图的绘制方法
2. **架构设计能力**：理解了三层架构的设计原则和职责划分
3. **面向对象分析**：学会了从需求中识别对象和关系的方法
4. **设计原则应用**：实践了SOLID设计原则，提高了代码质量
5. **流程分析能力**：通过活动图和顺序图，深入理解了业务流程

### 5.2 遇到的问题与解决方法

| 问题           | 解决方法                 |
| ------------ | -------------------- |
| Mermaid语法不熟悉 | 查阅Mermaid官方文档，学习语法规则 |
| 架构层次划分       | 参考三层架构设计模式，明确各层职责    |
| 状态转换逻辑       | 分析业务流程，确定状态之间的转换条件   |

### 5.3 改进方向

1. **完善异常处理**：在顺序图中添加异常处理流程
2. **优化性能**：考虑系统的性能瓶颈，优化数据访问
3. **增强安全性**：添加权限验证和安全措施
4. **扩展功能**：实现图书预约、续借等功能

***

## 六、思考题

### 6.1 为什么必须按用例图→活动图→架构设计→顺序图的顺序？

**回答**：

1. **用例图**：首先识别系统的参与者和功能需求，确定系统的边界和范围
2. **活动图**：分析业务流程，明确做什么，不考虑怎么做
3. **架构设计**：确定系统的分层结构，规划类的组织方式
4. **顺序图**：基于流程和架构，设计对象之间的交互细节

这个顺序确保了从宏观到微观、从业务到技术的逐步细化过程，避免了设计的盲目性和混乱性。

### 6.2 活动图和顺序图的区别是什么？

**回答**：

- **活动图**：关注流程视角，描述做什么，不关心谁来做，适合分析业务流程
- **顺序图**：关注对象视角，描述谁和谁交互，如何交互，适合设计对象间的协作

活动图是业务层面的分析工具，顺序图是技术层面的设计工具。

### 6.3 为什么要用Repository接口？

**回答**：

1. **依赖倒置**：依赖抽象而非具体实现，降低模块间耦合
2. **便于测试**：可以使用Mock对象进行单元测试
3. **可扩展性**：可以轻松替换数据库实现（如从MySQL改为PostgreSQL）
4. **代码清晰**：明确数据访问层的职责，符合单一职责原则

***

## 七、附录

### 7.1 项目结构

```
library/
├── week-04-实验报告.md
├── week-04-UML完整上机指南.md
└── src/
    ├── models/
    │   ├── user.rs
    │   ├── book.rs
    │   ├── borrow_record.rs
    │   └── fine_calculator.rs
    ├── services/
    │   ├── borrow_service.rs
    │   ├── return_service.rs
    │   └── fine_service.rs
    └── main.rs
```

### 7.2 Git提交信息

**提交命令**：

```bash
git add library/week-04-实验报告.md
git commit -m "experiment: submit week-04 UML report"
git push
```

**提交内容**：

- UML建模实验报告
- 所有UML图的Mermaid代码
- 设计说明文档

**分支信息**：

- 当前分支：master
- 远程仓库：origin (<https://gitee.com/yangqi-qiao/sqlrustgo.git>)

***

| 指导教师 | \_\_\_\_\_\_\_\_\_\_\_\_ | 实验成绩   | \_\_\_\_\_\_\_\_\_\_\_\_ |
| ---- | ------------------------ | ------ | ------------------------ |
| 批改日期 | \_\_\_\_\_\_\_\_\_\_\_\_ | <br /> | <br />                   |

