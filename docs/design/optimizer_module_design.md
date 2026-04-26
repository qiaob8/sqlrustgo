# Optimizer模块设计文档

## 1. 模块概述

Optimizer模块是SQLRustGo的核心组件之一，负责查询优化和执行计划生成。它接收来自Parser模块的抽象语法树（AST），经过逻辑优化和物理优化，生成高效的物理执行计划。

### 1.1 主要职责
- 逻辑优化：对逻辑执行计划进行优化
- 物理优化：生成和选择最佳物理执行计划
- 成本估算：计算执行计划的成本
- 索引选择：根据查询条件选择合适的索引

### 1.2 设计目标
- 生成高效的执行计划
- 支持各种SQL查询的优化
- 提供准确的成本估算
- 具有良好的可扩展性，支持新的优化策略

## 2. 核心功能

### 2.1 逻辑优化
- 常量折叠：计算常量表达式的值
- 谓词下推：将过滤条件下推到数据源
- 连接重排序：优化表连接的顺序
- 子查询优化：优化子查询的执行方式
- 表达式重写：重写复杂表达式以提高性能

### 2.2 物理优化
- 扫描方式选择：顺序扫描、索引扫描等
- 连接算法选择：嵌套循环连接、哈希连接、排序合并连接等
- 操作符选择：选择合适的操作符实现
- 并行执行策略：决定是否使用并行执行

### 2.3 成本估算
- 基数估计：估计结果集的大小
- 选择性估计：估计谓词的选择性
- 成本模型：计算执行计划的成本
- 统计信息管理：收集和维护表的统计信息

### 2.4 索引选择
- 索引匹配：根据查询条件匹配索引
- 索引评估：评估索引的有效性
- 索引推荐：推荐合适的索引

## 3. 类与接口设计

### 3.1 接口定义

#### Optimizer接口
```rust
pub trait Optimizer {
    fn optimize(&self, logical_plan: &LogicalPlan) -> Result<PhysicalPlan, OptimizeError>;
}
```

#### LogicalOptimizer接口
```rust
pub trait LogicalOptimizer {
    fn optimize(&self, logical_plan: &LogicalPlan) -> Result<LogicalPlan, OptimizeError>;
}
```

#### PhysicalOptimizer接口
```rust
pub trait PhysicalOptimizer {
    fn optimize(&self, logical_plan: &LogicalPlan) -> Result<PhysicalPlan, OptimizeError>;
}
```

#### CostModel接口
```rust
pub trait CostModel {
    fn calculate_cost(&self, plan: &PhysicalPlan) -> f64;
}
```

#### StatisticsProvider接口
```rust
pub trait StatisticsProvider {
    fn get_table_stats(&self, table_name: &str) -> Result<TableStatistics, StatisticsError>;
    fn get_column_stats(&self, table_name: &str, column_name: &str) -> Result<ColumnStatistics, StatisticsError>;
}
```

### 3.2 类定义

#### RuleBasedOptimizer类
```rust
pub struct RuleBasedOptimizer {
    rules: Vec<Box<dyn OptimizationRule>>,
}

impl LogicalOptimizer for RuleBasedOptimizer {
    fn optimize(&self, logical_plan: &LogicalPlan) -> Result<LogicalPlan, OptimizeError> {
        // 基于规则的逻辑优化实现
    }
}
```

#### CostBasedOptimizer类
```rust
pub struct CostBasedOptimizer {
    cost_model: Box<dyn CostModel>,
    statistics: Box<dyn StatisticsProvider>,
}

impl PhysicalOptimizer for CostBasedOptimizer {
    fn optimize(&self, logical_plan: &LogicalPlan) -> Result<PhysicalPlan, OptimizeError> {
        // 基于成本的物理优化实现
    }
}
```

#### OptimizationRule接口
```rust
pub trait OptimizationRule {
    fn apply(&self, plan: &mut LogicalPlan) -> bool;
}
```

#### TableStatistics结构体
```rust
pub struct TableStatistics {
    pub row_count: f64,
    pub data_size: f64,
    pub column_stats: HashMap<String, ColumnStatistics>,
}
```

#### ColumnStatistics结构体
```rust
pub struct ColumnStatistics {
    pub distinct_count: f64,
    pub min_value: Option<Value>,
    pub max_value: Option<Value>,
    pub null_count: f64,
}
```

## 4. 执行流程

### 4.1 逻辑优化流程
1. 接收逻辑执行计划
2. 应用一系列逻辑优化规则
3. 生成优化后的逻辑执行计划
4. 返回优化后的逻辑执行计划

### 4.2 物理优化流程
1. 接收优化后的逻辑执行计划
2. 生成候选物理执行计划
3. 计算每个候选计划的成本
4. 选择成本最低的物理执行计划
5. 返回最佳物理执行计划

### 4.3 成本估算流程
1. 收集表和列的统计信息
2. 估计操作符的输入和输出大小
3. 计算每个操作符的成本
4. 累加操作符的成本得到总执行成本

## 5. 异常处理

### 5.1 优化错误
- OptimizeError：优化过程中的错误
- LogicalOptimizeError：逻辑优化错误
- PhysicalOptimizeError：物理优化错误
- CostEstimationError：成本估算错误

### 5.2 统计信息错误
- StatisticsError：统计信息相关错误
- TableStatsNotFoundError：表统计信息不存在
- ColumnStatsNotFoundError：列统计信息不存在

## 6. 性能考虑

### 6.1 优化策略选择
- 针对不同类型的查询选择合适的优化策略
- 平衡优化时间和执行时间
- 避免过度优化

### 6.2 成本模型优化
- 设计准确的成本模型
- 考虑CPU、IO、内存等因素
- 适应不同的硬件环境

### 6.3 统计信息管理
- 定期收集和更新统计信息
- 支持增量统计信息收集
- 缓存统计信息以提高性能

### 6.4 并行优化
- 支持并行优化大型查询
- 利用多线程加速优化过程
- 平衡优化并行度和资源消耗

## 7. 测试策略

### 7.1 单元测试
- 测试逻辑优化规则
- 测试物理优化策略
- 测试成本估算模型
- 测试统计信息管理

### 7.2 集成测试
- 测试完整的优化流程
- 测试各种查询的优化结果
- 测试优化器与其他模块的集成
- 测试边界情况和特殊查询

### 7.3 性能测试
- 测试优化器的性能
- 测试优化后执行计划的性能
- 测试不同规模查询的优化时间
- 测试优化器的内存使用情况

## 8. 扩展与维护

### 8.1 扩展优化规则
- 添加新的逻辑优化规则
- 添加新的物理优化策略
- 扩展成本模型
- 支持新的索引类型

### 8.2 维护建议
- 定期更新优化规则
- 优化成本模型
- 改进统计信息收集
- 添加新的测试用例

## 9. 总结

Optimizer模块是SQLRustGo的重要组成部分，负责生成高效的执行计划。通过合理的设计和实现，Optimizer模块能够显著提高查询性能，减少资源消耗。

未来的改进方向包括：
- 实现更复杂的优化策略
- 支持更多类型的查询优化
- 提高成本估算的准确性
- 增强统计信息管理
- 支持并行查询优化