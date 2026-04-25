use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, PartialEq)]
pub enum LockType {
    Shared,
    Exclusive,
}

#[derive(Debug)]
struct Lock {
    lock_type: LockType,
    holder: Option<u64>,
    ref_count: u32,
}

pub struct LockManager {
    locks: RwLock<HashMap<String, Lock>>,
}

impl LockManager {
    pub fn new() -> Self {
        Self {
            locks: RwLock::new(HashMap::new()),
        }
    }
    
    pub fn acquire_lock(&self, tx_id: u64, resource: &str, lock_type: LockType) -> Result<(), String> {
        let mut locks = self.locks.write().map_err(|e| format!("Lock error: {}", e))?;
        
        match locks.get_mut(resource) {
            Some(lock) => {
                if lock.holder == Some(tx_id) {
                    lock.ref_count += 1;
                    return Ok(());
                }
                
                if lock.holder.is_some() && lock.holder != Some(tx_id) {
                    return Err(format!("Resource '{}' is locked by another transaction", resource));
                }
                
                if lock.lock_type == LockType::Exclusive && lock_type == LockType::Shared {
                    return Err(format!("Cannot acquire shared lock on exclusive resource '{}'", resource));
                }
                
                lock.ref_count += 1;
                lock.holder = Some(tx_id);
                lock.lock_type = lock_type;
                
                Ok(())
            }
            None => {
                locks.insert(resource.to_string(), Lock {
                    lock_type: lock_type.clone(),
                    holder: Some(tx_id),
                    ref_count: 1,
                });
                Ok(())
            }
        }
    }
    
    pub fn release_lock(&self, tx_id: u64, resource: &str) -> Result<(), String> {
        let mut locks = self.locks.write().map_err(|e| format!("Lock error: {}", e))?;
        
        match locks.get_mut(resource) {
            Some(lock) => {
                if lock.holder != Some(tx_id) {
                    return Err(format!("Transaction {} does not hold lock on '{}'", tx_id, resource));
                }
                
                lock.ref_count -= 1;
                if lock.ref_count == 0 {
                    locks.remove(resource);
                }
                
                Ok(())
            }
            None => Ok(()),
        }
    }
}
