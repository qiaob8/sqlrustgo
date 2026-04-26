# Storage模块设计文档

## 1. 模块概述

Storage模块是SQLRustGo的核心组件之一，负责数据的存储和管理。它提供了统一的存储接口，支持内存存储和文件存储两种实现方式，为Executor模块提供数据访问能力。

### 1.1 主要职责
- 数据存储与检索
- 表结构管理
- 事务处理
- 并发控制
- 缓存管理

### 1.2 设计目标
- 提供高效的数据存储和检索
- 支持事务的ACID特性
- 提供统一的存储接口
- 具有良好的可扩展性，支持新的存储引擎实现

## 2. 核心功能

### 2.1 数据存储与检索
- 读取数据：根据键读取数据
- 写入数据：存储新数据或更新现有数据
- 删除数据：删除指定数据
- 扫描数据：扫描表中的数据

### 2.2 表结构管理
- 创建表：创建新表
- 删除表：删除现有表
- 修改表：修改表结构
- 获取表信息：获取表的元数据

### 2.3 事务处理
- 开始事务：启动新事务
- 提交事务：提交事务的更改
- 回滚事务：撤销事务的更改
- 事务隔离：确保事务的隔离性

### 2.4 并发控制
- 锁管理：管理数据的锁定
- 并发访问控制：控制多个事务的并发访问
- 死锁检测：检测和处理死锁

### 2.5 缓存管理
- 缓冲池：管理内存中的数据页
- 缓存策略：实现LRU等缓存替换策略
- 预读取：提前读取可能需要的数据

## 3. 类与接口设计

### 3.1 接口定义

#### StorageEngine接口
```rust
pub trait StorageEngine {
    fn read(&mut self, table_name: &str, key: &[u8]) -> Result<Option<Vec<u8>>, StorageError>;
    fn write(&mut self, table_name: &str, key: &[u8], value: &[u8]) -> Result<(), StorageError>;
    fn delete(&mut self, table_name: &str, key: &[u8]) -> Result<(), StorageError>;
    fn scan(&mut self, table_name: &str) -> Result<Box<dyn Iterator<Item = (Vec<u8>, Vec<u8>)> + '_>, StorageError>;
    fn create_table(&mut self, table_name: &str) -> Result<(), StorageError>;
    fn drop_table(&mut self, table_name: &str) -> Result<(), StorageError>;
    fn begin_transaction(&mut self) -> Result<TransactionId, StorageError>;
    fn commit_transaction(&mut self, tx_id: TransactionId) -> Result<(), StorageError>;
    fn rollback_transaction(&mut self, tx_id: TransactionId) -> Result<(), StorageError>;
}
```

### 3.2 类定义

#### MemoryStorage类
```rust
pub struct MemoryStorage {
    tables: HashMap<String, HashMap<Vec<u8>, Vec<u8>>>,
    transactions: HashMap<TransactionId, Transaction>,
    lock_manager: Arc<LockManager>,
}

impl StorageEngine for MemoryStorage {
    fn read(&mut self, table_name: &str, key: &[u8]) -> Result<Option<Vec<u8>>, StorageError> {
        // 内存存储读取实现
    }
    
    fn write(&mut self, table_name: &str, key: &[u8], value: &[u8]) -> Result<(), StorageError> {
        // 内存存储写入实现
    }
    
    // 其他方法实现...
}
```

#### FileStorage类
```rust
pub struct FileStorage {
    data_dir: String,
    buffer_pool: BufferPool,
    tables: HashMap<String, Table>,
    transactions: HashMap<TransactionId, Transaction>,
    lock_manager: Arc<LockManager>,
}

impl StorageEngine for FileStorage {
    fn read(&mut self, table_name: &str, key: &[u8]) -> Result<Option<Vec<u8>>, StorageError> {
        // 文件存储读取实现
    }
    
    fn write(&mut self, table_name: &str, key: &[u8], value: &[u8]) -> Result<(), StorageError> {
        // 文件存储写入实现
    }
    
    // 其他方法实现...
}
```

#### BufferPool类
```rust
pub struct BufferPool {
    frames: Vec<Option<Page>>,
    lru: LruCache<PageId, usize>,
    capacity: usize,
}

impl BufferPool {
    pub fn get_page(&mut self, page_id: PageId) -> Result<&Page, BufferPoolError> {
        // 获取页面
    }
    
    pub fn put_page(&mut self, page: Page) -> Result<(), BufferPoolError> {
        // 存储页面
    }
    
    pub fn flush_all(&mut self) -> Result<(), BufferPoolError> {
        // 刷新所有页面到磁盘
    }
}
```

#### LockManager类
```rust
pub struct LockManager {
    locks: HashMap<(String, Vec<u8>), Lock>,
    waiting_transactions: HashMap<TransactionId, Vec<LockRequest>>,
}

impl LockManager {
    pub fn acquire_lock(&mut self, tx_id: TransactionId, table_name: &str, key: &[u8], lock_type: LockType) -> Result<(), LockError> {
        // 获取锁
    }
    
    pub fn release_locks(&mut self, tx_id: TransactionId) -> Result<(), LockError> {
        // 释放锁
    }
    
    pub fn detect_deadlock(&mut self) -> Option<TransactionId> {
        // 检测死锁
    }
}
```

### 3.3 数据结构

#### Page结构体
```rust
pub struct Page {
    page_id: PageId,
    data: Vec<u8>,
    dirty: bool,
    pin_count: usize,
}
```

#### Table结构体
```rust
pub struct Table {
    name: String,
    schema: TableSchema,
    pages: HashMap<PageId, Page>,
    next_page_id: PageId,
}
```

#### Transaction结构体
```rust
pub struct Transaction {
    id: TransactionId,
    status: TransactionStatus,
    start_time: Instant,
    changes: Vec<TransactionChange>,
}
```

#### Lock结构体
```rust
pub struct Lock {
    lock_type: LockType,
    holding_transactions: Vec<TransactionId>,
    waiting_transactions: Vec<TransactionId>,
}
```

## 4. 执行流程

### 4.1 数据读取流程
1. 接收读取请求
2. 检查事务状态
3. 获取锁
4. 从缓存或磁盘读取数据
5. 释放锁
6. 返回数据

### 4.2 数据写入流程
1. 接收写入请求
2. 检查事务状态
3. 获取锁
4. 写入数据到缓存
5. 标记页面为脏页
6. 释放锁
7. 返回成功

### 4.3 事务处理流程
1. 开始事务：创建事务对象，分配事务ID
2. 执行操作：执行各种数据操作
3. 提交事务：将更改持久化到磁盘，释放锁
4. 回滚事务：撤销更改，释放锁

### 4.4 缓冲池管理流程
1. 请求页面：检查页面是否在缓冲池中
2. 如果页面在缓冲池：返回页面，增加pin计数
3. 如果页面不在缓冲池：选择要替换的页面，读取页面到缓冲池
4. 使用页面：执行读写操作
5. 释放页面：减少pin计数，根据需要刷新脏页

## 5. 异常处理

### 5.1 存储错误
- StorageError：存储操作错误
- TableNotFoundError：表不存在
- KeyNotFoundError：键不存在
- IOError：I/O操作错误

### 5.2 事务错误
- TransactionError：事务错误
- TransactionAbortedError：事务被中止
- TransactionTimeoutError：事务超时
- DeadlockError：死锁错误

### 5.3 锁错误
- LockError：锁错误
- LockTimeoutError：锁超时
- LockConflictError：锁冲突

### 5.4 缓冲池错误
- BufferPoolError：缓冲池错误
- BufferPoolFullError：缓冲池满
- PageNotFoundError：页面不存在

## 6. 性能考虑

### 6.1 存储引擎选择
- 根据使用场景选择合适的存储引擎
- 内存存储：适合临时数据和测试
- 文件存储：适合持久化数据

### 6.2 缓存优化
- 合理设置缓冲池大小
- 选择合适的缓存替换策略
- 实现预读取和预写入

### 6.3 并发控制优化
- 选择合适的锁粒度
- 实现锁升级和降级
- 优化死锁检测算法

### 6.4 I/O优化
- 减少磁盘I/O次数
- 实现批量读写
- 使用异步I/O

## 7. 测试策略

### 7.1 单元测试
- 测试存储引擎的基本功能
- 测试事务处理
- 测试并发控制
- 测试缓冲池管理

### 7.2 集成测试
- 测试存储引擎与其他模块的集成
- 测试完整的存储流程
- 测试边界情况和特殊输入

### 7.3 性能测试
- 测试存储引擎的性能
- 测试不同规模数据的存储性能
- 测试并发访问的性能
- 测试缓冲池的性能

## 8. 扩展与维护

### 8.1 扩展存储引擎
- 添加新的存储引擎实现
- 扩展现有存储引擎的功能
- 支持新的存储介质

### 8.2 维护建议
- 定期备份数据
- 监控存储性能
- 优化存储配置
- 处理存储故障

## 9. 总结

Storage模块是SQLRustGo的重要组成部分，负责数据的存储和管理。通过合理的设计和实现，Storage模块能够提供高效、可靠的数据存储服务，支持事务处理和并发控制。

未来的改进方向包括：
- 实现更多存储引擎类型
- 优化存储性能
- 增强事务处理能力
- 支持更多存储特性
- 提高存储可靠性