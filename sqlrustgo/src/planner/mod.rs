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
        match statement {
            Statement::Select { table_name, columns, where_clause } => {
                Ok(PhysicalPlan::Scan {
                    table_name: table_name.clone(),
                    columns: columns.clone(),
                    filter: where_clause.clone(),
                })
            }
            Statement::Insert { table_name, values } => {
                Ok(PhysicalPlan::Insert {
                    table_name: table_name.clone(),
                    values: values.clone(),
                })
            }
            Statement::Update { table_name, set_clauses, where_clause } => {
                Ok(PhysicalPlan::Update {
                    table_name: table_name.clone(),
                    set_clauses: set_clauses.clone(),
                    filter: where_clause.clone(),
                })
            }
            Statement::Delete { table_name, where_clause } => {
                Ok(PhysicalPlan::Delete {
                    table_name: table_name.clone(),
                    filter: where_clause.clone(),
                })
            }
            Statement::CreateTable { table_name, columns } => {
                Ok(PhysicalPlan::CreateTable {
                    table_name: table_name.clone(),
                    columns: columns.clone(),
                })
            }
        }
    }
}
