use crate::planner::PhysicalPlan;
use crate::parser::ast::{self, Expr, Value};
use crate::storage::StorageEngine;
use crate::catalog::Catalog;
use crate::executor::operators::Record;

pub struct Executor;

impl Executor {
    pub fn new() -> Self {
        Self
    }
    
    pub fn execute(
        &self,
        plan: PhysicalPlan,
        storage: &mut dyn StorageEngine,
        catalog: &Catalog,
    ) -> Result<String, String> {
        match plan {
            PhysicalPlan::Scan { table_name, columns, filter } => {
                self.execute_scan(&table_name, &columns, filter, storage, catalog)
            }
            PhysicalPlan::Project { input, columns } => {
                self.execute_project(*input, &columns, storage, catalog)
            }
            PhysicalPlan::Filter { input, condition } => {
                self.execute_filter(*input, condition, storage, catalog)
            }
            PhysicalPlan::Insert { table_name, values } => {
                self.execute_insert(&table_name, values, storage, catalog)
            }
            PhysicalPlan::Update { table_name, set_clauses, filter } => {
                self.execute_update(&table_name, set_clauses, filter, storage, catalog)
            }
            PhysicalPlan::Delete { table_name, filter } => {
                self.execute_delete(&table_name, filter, storage, catalog)
            }
            PhysicalPlan::CreateTable { table_name, columns } => {
                self.execute_create_table(&table_name, columns, catalog)
            }
        }
    }
    
    fn execute_scan(
        &self,
        table_name: &str,
        _columns: &[String],
        filter: Option<Expr>,
        storage: &mut dyn StorageEngine,
        catalog: &Catalog,
    ) -> Result<String, String> {
        let table = catalog.get_table(table_name)
            .map_err(|e| format!("Failed to get table: {}", e))?;
        
        if table.is_none() {
            return Err(format!("Table '{}' not found", table_name));
        }
        
        let records = storage.scan(table_name)
            .map_err(|e| format!("Failed to scan table: {}", e))?;
        
        let mut result = Vec::new();
        for (key, value) in records {
            let record: Record = serde_json::from_slice(&value)
                .map_err(|e| format!("Failed to deserialize record: {}", e))?;
            
            if let Some(ref expr) = filter {
                if self.evaluate_filter(expr, &record)? {
                    result.push(record.clone());
                }
            } else {
                result.push(record);
            }
        }
        
        Ok(format!("Scanned {} records from '{}'", result.len(), table_name))
    }
    
    fn execute_project(
        &self,
        input: PhysicalPlan,
        columns: &[String],
        storage: &mut dyn StorageEngine,
        catalog: &Catalog,
    ) -> Result<String, String> {
        // 先执行输入计划
        let input_result = self.execute(input, storage, catalog)?;
        
        // 投影操作的结果
        Ok(format!("Projected columns: {:?}, {}", columns, input_result))
    }
    
    fn execute_filter(
        &self,
        input: PhysicalPlan,
        condition: Expr,
        storage: &mut dyn StorageEngine,
        catalog: &Catalog,
    ) -> Result<String, String> {
        // 先执行输入计划
        let input_result = self.execute(input, storage, catalog)?;
        
        // 过滤操作的结果
        Ok(format!("Filtered with condition, {}", input_result))
    }
    
    fn execute_insert(
        &self,
        table_name: &str,
        values: Vec<Value>,
        storage: &mut dyn StorageEngine,
        catalog: &Catalog,
    ) -> Result<String, String> {
        let table = catalog.get_table(table_name)
            .map_err(|e| format!("Failed to get table: {}", e))?;
        
        if table.is_none() {
            return Err(format!("Table '{}' not found", table_name));
        }
        
        let record = Record {
            values: values.into_iter().map(|v| match v {
                Value::Number(n) => serde_json::Value::Number(n.into()),
                Value::String(s) => serde_json::Value::String(s),
            }).collect(),
        };
        
        let key = format!("record_{}", uuid::Uuid::new_v4());
        let value = serde_json::to_vec(&record)
            .map_err(|e| format!("Failed to serialize record: {}", e))?;
        
        storage.write(table_name, key.as_bytes(), &value)
            .map_err(|e| format!("Failed to write record: {}", e))?;
        
        Ok("1 row inserted".to_string())
    }
    
    fn execute_update(
        &self,
        table_name: &str,
        set_clauses: Vec<(String, Value)>,
        filter: Option<Expr>,
        storage: &mut dyn StorageEngine,
        catalog: &Catalog,
    ) -> Result<String, String> {
        let records = storage.scan(table_name)
            .map_err(|e| format!("Failed to scan table: {}", e))?;
        
        let mut updated_count = 0;
        for (key, value) in records {
            let mut record: Record = serde_json::from_slice(&value)
                .map_err(|e| format!("Failed to deserialize record: {}", e))?;
            
            if let Some(ref expr) = filter {
                if !self.evaluate_filter(expr, &record)? {
                    continue;
                }
            }
            
            for (column, value) in &set_clauses {
                record.values.insert(column.clone(), match value {
                    Value::Number(n) => serde_json::Value::Number((*n).into()),
                    Value::String(s) => serde_json::Value::String(s.clone()),
                });
            }
            
            let new_value = serde_json::to_vec(&record)
                .map_err(|e| format!("Failed to serialize record: {}", e))?;
            
            storage.write(table_name, &key, &new_value)
                .map_err(|e| format!("Failed to update record: {}", e))?;
            
            updated_count += 1;
        }
        
        Ok(format!("{} rows updated", updated_count))
    }
    
    fn execute_delete(
        &self,
        table_name: &str,
        filter: Option<Expr>,
        storage: &mut dyn StorageEngine,
        catalog: &Catalog,
    ) -> Result<String, String> {
        let records = storage.scan(table_name)
            .map_err(|e| format!("Failed to scan table: {}", e))?;
        
        let mut deleted_count = 0;
        for (key, value) in records {
            let record: Record = serde_json::from_slice(&value)
                .map_err(|e| format!("Failed to deserialize record: {}", e))?;
            
            if let Some(ref expr) = filter {
                if !self.evaluate_filter(expr, &record)? {
                    continue;
                }
            }
            
            storage.delete(table_name, &key)
                .map_err(|e| format!("Failed to delete record: {}", e))?;
            
            deleted_count += 1;
        }
        
        Ok(format!("{} rows deleted", deleted_count))
    }
    
    fn execute_create_table(
        &self,
        table_name: &str,
        _columns: Vec<ast::ColumnDefinition>,
        catalog: &mut Catalog,
    ) -> Result<String, String> {
        catalog.create_table(table_name, vec![])
            .map_err(|e| format!("Failed to create table: {}", e))?;
        
        Ok(format!("Table '{}' created", table_name))
    }
    
    fn evaluate_filter(&self, expr: &Expr, record: &Record) -> Result<bool, String> {
        match expr {
            Expr::Equal(left, right) => {
                let left_val = self.evaluate_expr(left, record)?;
                let right_val = self.evaluate_expr(right, record)?;
                Ok(left_val == right_val)
            }
            Expr::LessThan(left, right) => {
                let left_val = self.evaluate_expr(left, record)?;
                let right_val = self.evaluate_expr(right, record)?;
                Ok(left_val < right_val)
            }
            Expr::GreaterThan(left, right) => {
                let left_val = self.evaluate_expr(left, record)?;
                let right_val = self.evaluate_expr(right, record)?;
                Ok(left_val > right_val)
            }
            Expr::Column(name) => {
                let val = record.values.get(name)
                    .ok_or(format!("Column '{}' not found", name))?;
                Ok(val.is_number() || val.as_str().is_some())
            }
            Expr::Value(_) => Ok(true),
        }
    }
    
    fn evaluate_expr(&self, expr: &Expr, record: &Record) -> Result<serde_json::Value, String> {
        match expr {
            Expr::Column(name) => {
                record.values.get(name)
                    .cloned()
                    .ok_or(format!("Column '{}' not found", name))
            }
            Expr::Value(Value::Number(n)) => Ok(serde_json::Value::Number((*n).into())),
            Expr::Value(Value::String(s)) => Ok(serde_json::Value::String(s.clone())),
            _ => Err("Unsupported expression".to_string()),
        }
    }
}
