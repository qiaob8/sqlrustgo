pub mod logical_plan;
pub mod physical_plan;

pub use logical_plan::LogicalPlan;
pub use physical_plan::PhysicalPlan;

use crate::parser::{Statement, ast};
use crate::catalog::Catalog;

pub struct Planner;

impl Planner {
    pub fn new() -> Self {
        Self
    }
    
    pub fn plan(&self, statement: &Statement, _catalog: &Catalog) -> Result<PhysicalPlan, String> {
        // 从SQL语句创建逻辑计划
        let logical_plan = LogicalPlan::from_statement(statement);
        
        // 优化逻辑计划
        let optimized_logical_plan = logical_plan.optimize();
        
        // 将逻辑计划转换为物理计划
        self.logical_to_physical(&optimized_logical_plan)
    }
    
    fn logical_to_physical(&self, logical_plan: &LogicalPlan) -> Result<PhysicalPlan, String> {
        match logical_plan {
            LogicalPlan::Scan { table_name, columns, filter } => {
                Ok(PhysicalPlan::Scan {
                    table_name: table_name.clone(),
                    columns: columns.clone(),
                    filter: filter.clone(),
                })
            }
            LogicalPlan::Project { input, columns } => {
                let physical_input = self.logical_to_physical(input)?;
                Ok(PhysicalPlan::Project {
                    input: Box::new(physical_input),
                    columns: columns.clone(),
                })
            }
            LogicalPlan::Filter { input, condition } => {
                let physical_input = self.logical_to_physical(input)?;
                Ok(PhysicalPlan::Filter {
                    input: Box::new(physical_input),
                    condition: condition.clone(),
                })
            }
            LogicalPlan::Insert { table_name, values } => {
                Ok(PhysicalPlan::Insert {
                    table_name: table_name.clone(),
                    values: values.clone(),
                })
            }
            LogicalPlan::Update { table_name, set_clauses, filter } => {
                Ok(PhysicalPlan::Update {
                    table_name: table_name.clone(),
                    set_clauses: set_clauses.clone(),
                    filter: filter.clone(),
                })
            }
            LogicalPlan::Delete { table_name, filter } => {
                Ok(PhysicalPlan::Delete {
                    table_name: table_name.clone(),
                    filter: filter.clone(),
                })
            }
            LogicalPlan::CreateTable { table_name, columns } => {
                Ok(PhysicalPlan::CreateTable {
                    table_name: table_name.clone(),
                    columns: columns.clone(),
                })
            }
        }
    }
    
    pub fn optimize_physical(&self, plan: &PhysicalPlan) -> PhysicalPlan {
        // 简单的物理优化
        match plan {
            PhysicalPlan::Scan { table_name, columns, filter } => {
                // 如果有过滤条件，创建Filter节点
                if let Some(filter_expr) = filter {
                    PhysicalPlan::Filter {
                        input: Box::new(PhysicalPlan::Scan {
                            table_name: table_name.clone(),
                            columns: columns.clone(),
                            filter: None,
                        }),
                        condition: filter_expr.clone(),
                    }
                } else {
                    plan.clone()
                }
            }
            PhysicalPlan::Filter { input, condition } => {
                let optimized_input = self.optimize_physical(input);
                PhysicalPlan::Filter {
                    input: Box::new(optimized_input),
                    condition: condition.clone(),
                }
            }
            PhysicalPlan::Project { input, columns } => {
                let optimized_input = self.optimize_physical(input);
                PhysicalPlan::Project {
                    input: Box::new(optimized_input),
                    columns: columns.clone(),
                }
            }
            _ => plan.clone(),
        }
    }
}
