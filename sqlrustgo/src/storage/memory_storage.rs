use std::collections::HashMap;
use std::sync::RwLock;
use super::StorageEngine;

pub struct MemoryStorage {
    data: RwLock<HashMap<String, HashMap<Vec<u8>, Vec<u8>>>>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self {
            data: RwLock::new(HashMap::new()),
        }
    }
}

impl StorageEngine for MemoryStorage {
    fn read(&self, table_name: &str, key: &[u8]) -> Result<Option<Vec<u8>>, String> {
        let data = self.data.read().map_err(|e| format!("Lock error: {}", e))?;
        
        match data.get(table_name) {
            Some(table_data) => {
                Ok(table_data.get(key).cloned())
            }
            None => Ok(None),
        }
    }
    
    fn write(&mut self, table_name: &str, key: &[u8], value: &[u8]) -> Result<(), String> {
        let mut data = self.data.write().map_err(|e| format!("Lock error: {}", e))?;
        
        let table_data = data.entry(table_name.to_string()).or_insert_with(HashMap::new);
        table_data.insert(key.to_vec(), value.to_vec());
        
        Ok(())
    }
    
    fn delete(&mut self, table_name: &str, key: &[u8]) -> Result<(), String> {
        let mut data = self.data.write().map_err(|e| format!("Lock error: {}", e))?;
        
        if let Some(table_data) = data.get_mut(table_name) {
            table_data.remove(key);
        }
        
        Ok(())
    }
    
    fn scan(&self, table_name: &str) -> Result<Box<dyn Iterator<Item = (Vec<u8>, Vec<u8>)> + '_>, String> {
        let data = self.data.read().map_err(|e| format!("Lock error: {}", e))?;
        
        match data.get(table_name) {
            Some(table_data) => {
                let iter = table_data.iter().map(|(k, v)| (k.clone(), v.clone()));
                Ok(Box::new(iter))
            }
            None => Ok(Box::new(std::iter::empty())),
        }
    }
}
