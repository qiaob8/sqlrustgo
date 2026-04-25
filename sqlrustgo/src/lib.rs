// SQLRustGo 1.0 数据库系统
// 分层架构实现

pub mod parser;
pub mod planner;
pub mod executor;
pub mod storage;
pub mod catalog;
pub mod transaction;

/// SQLRustGo 数据库系统
pub struct SqlRustGo {
    pub parser: parser::Parser,
    pub planner: planner::Planner,
    pub executor: executor::Executor,
    pub storage: Box<dyn storage::StorageEngine>,
    pub catalog: catalog::Catalog,
    pub transaction_manager: transaction::TransactionManager,
}

impl SqlRustGo {
    /// 创建一个新的SQLRustGo实例
    pub fn new() -> Self {
        let catalog = catalog::Catalog::new();
        let storage = Box::new(storage::MemoryStorage::new());
        let transaction_manager = transaction::TransactionManager::new();
        
        Self {
            parser: parser::Parser::new(),
            planner: planner::Planner::new(),
            executor: executor::Executor::new(),
            storage,
            catalog,
            transaction_manager,
        }
    }
    
    /// 执行SQL语句
    pub fn execute(&mut self, sql: &str) -> Result<String, String> {
        // 解析SQL
        let ast = self.parser.parse(sql)?;
        
        // 生成执行计划
        let plan = self.planner.plan(&ast, &self.catalog)?;
        
        // 执行计划
        let result = self.executor.execute(plan, &mut *self.storage, &self.catalog)?;
        
        Ok(result)
    }
}
