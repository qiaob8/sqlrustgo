# Executor模块设计文档

## 1. 模块概述

Executor模块是SQLRustGo的核心组件之一，负责执行物理执行计划并返回结果。它接收来自Optimizer模块的物理执行计划，构建执行算子树，执行查询，并处理执行结果。

### 1.1 主要职责
- 执行物理执行计划
- 构建和管理执行算子树
- 处理查询结果
- 管理执行上下文
- 处理并发执行

### 1.2 设计目标
- 高效执行查询
- 支持各种SQL操作的执行
- 提供灵活的执行策略
- 具有良好的可扩展性，支持新的执行算子和执行策略

## 2. 核心功能

### 2.1 执行计划执行
- 构建执行算子树
- 初始化执行上下文
- 执行算子树
- 处理执行结果

### 2.2 算子管理
- 扫描算子：扫描表数据
- 投影算子：选择指定列
- 过滤算子：应用过滤条件
- 连接算子：连接多个表
- 聚合算子：执行聚合操作

### 2.3 结果处理
- 结果集构建
- 结果格式化
- 结果返回

### 2.4 并发控制
- 事务管理
- 锁管理
- 并发执行策略

## 3. 类与接口设计

### 3.1 接口定义

#### Executor接口
```rust
pub trait Executor {
    fn execute(&self, plan: &PhysicalPlan) -> Result<ResultSet, ExecutorError>;
}
```

#### Operator接口
```rust
pub trait Operator {
    fn open(&mut self) -> Result<(), OperatorError>;
    fn next(&mut self) -> Result<Option<RecordBatch>, OperatorError>;
    fn close(&mut self) -> Result<(), OperatorError>;
}
```

### 3.2 类定义

#### SimpleExecutor类
```rust
pub struct SimpleExecutor {
    storage: Box<dyn StorageEngine>,
    catalog: Arc<Catalog>,
}

impl Executor for SimpleExecutor {
    fn execute(&self, plan: &PhysicalPlan) -> Result<ResultSet, ExecutorError> {
        // 执行物理执行计划
    }
}

impl SimpleExecutor {
    fn build_operator(&self, plan: &PhysicalPlan) -> Result<Box<dyn Operator>, ExecutorError> {
        // 构建执行算子
    }
}
```

#### ScanOperator类
```rust
pub struct ScanOperator {
    storage: Box<dyn StorageEngine>,
    table: String,
    filter: Option<Expr>,
    iterator: Option<Box<dyn Iterator<Item = (Vec<u8>, Vec<u8>)>>>,
}

impl Operator for ScanOperator {
    fn open(&mut self) -> Result<(), OperatorError> {
        // 打开扫描操作
    }
    
    fn next(&mut self) -> Result<Option<RecordBatch>, OperatorError> {
        // 获取下一批数据
    }
    
    fn close(&mut self) -> Result<(), OperatorError> {
        // 关闭扫描操作
    }
}
```

#### ProjectOperator类
```rust
pub struct ProjectOperator {
    child: Box<dyn Operator>,
    projections: Vec<Expr>,
}

impl Operator for ProjectOperator {
    fn open(&mut self) -> Result<(), OperatorError> {
        // 打开投影操作
    }
    
    fn next(&mut self) -> Result<Option<RecordBatch>, OperatorError> {
        // 执行投影操作
    }
    
    fn close(&mut self) -> Result<(), OperatorError> {
        // 关闭投影操作
    }
}
```

#### FilterOperator类
```rust
pub struct FilterOperator {
    child: Box<dyn Operator>,
    predicate: Expr,
}

impl Operator for FilterOperator {
    fn open(&mut self) -> Result<(), OperatorError> {
        // 打开过滤操作
    }
    
    fn next(&mut self) -> Result<Option<RecordBatch>, OperatorError> {
        // 执行过滤操作
    }
    
    fn close(&mut self) -> Result<(), OperatorError> {
        // 关闭过滤操作
    }
}
```

### 3.3 数据结构

#### ResultSet结构体
```rust
pub struct ResultSet {
    columns: Vec<String>,
    rows: Vec<Vec<Value>>,
}
```

#### RecordBatch结构体
```rust
pub struct RecordBatch {
    records: Vec<Record>,
}
```

#### Record结构体
```rust
pub struct Record {
    values: HashMap<String, serde_json::Value>,
}
```

#### ExecutionContext结构体
```rust
pub struct ExecutionContext {
    transaction_id: Option<TransactionId>,
    parameters: HashMap<String, Value>,
    statistics: ExecutionStatistics,
}
```

## 4. 执行流程

### 4.1 执行计划执行流程
1. 接收物理执行计划
2. 构建执行算子树
3. 初始化执行上下文
4. 打开根算子
5. 迭代获取结果
6. 关闭算子树
7. 构建并返回结果集

### 4.2 算子执行流程
1. 打开算子（open）
2. 迭代获取数据（next）
3. 处理数据
4. 关闭算子（close）

### 4.3 结果处理流程
1. 收集执行结果
2. 格式化结果
3. 构建结果集
4. 返回结果集

## 5. 异常处理

### 5.1 执行错误
- ExecutorError：执行过程中的错误
- OperatorError：算子执行错误
- ExecutionTimeoutError：执行超时
- ResourceExhaustedError：资源耗尽

### 5.2 存储错误
- StorageError：存储操作错误
- TransactionError：事务错误
- LockError：锁错误

### 5.3 数据错误
- DataError：数据相关错误
- TypeMismatchError：类型不匹配
- ConstraintViolationError：约束违反

## 6. 性能考虑

### 6.1 执行策略优化
- 选择合适的执行策略
- 优化算子执行顺序
- 利用并行执行

### 6.2 内存管理
- 合理使用内存
- 避免内存泄漏
- 优化内存使用模式

### 6.3 I/O优化
- 减少磁盘I/O
- 优化数据读取策略
- 使用预读取和缓存

### 6.4 并发优化
- 合理的并发控制策略
- 减少锁竞争
- 提高并发度

## 7. 测试策略

### 7.1 单元测试
- 测试各个算子的执行
- 测试执行器的基本功能
- 测试结果处理
- 测试异常处理

### 7.2 集成测试
- 测试完整的执行流程
- 测试各种SQL操作的执行
- 测试执行器与其他模块的集成
- 测试边界情况和特殊查询

### 7.3 性能测试
- 测试执行器的性能
- 测试不同规模数据的执行时间
- 测试并发执行的性能
- 测试内存使用情况

## 8. 扩展与维护

### 8.1 扩展执行算子
- 添加新的执行算子
- 扩展现有算子的功能
- 支持新的执行策略

### 8.2 维护建议
- 优化执行策略
- 改进内存管理
- 增强错误处理
- 添加新的测试用例

## 9. 总结

Executor模块是SQLRustGo的重要组成部分，负责执行物理执行计划并返回结果。通过合理的设计和实现，Executor模块能够高效地执行各种SQL操作，处理复杂的查询。

未来的改进方向包括：
- 实现更复杂的执行算子
- 支持更多类型的SQL操作
- 提高执行性能
- 增强并发执行能力
- 优化内存使用