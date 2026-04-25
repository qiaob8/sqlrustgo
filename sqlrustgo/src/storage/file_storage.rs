use std::collections::HashMap;
use std::sync::RwLock;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use super::StorageEngine;

pub struct FileStorage {
    data: RwLock<HashMap<String, HashMap<Vec<u8>, Vec<u8>>>>,
    base_path: String,
}

impl FileStorage {
    pub fn new(base_path: &str) -> Self {
        fs::create_dir_all(base_path).ok();
        
        Self {
            data: RwLock::new(HashMap::new()),
            base_path: base_path.to_string(),
        }
    }
    
    fn get_table_file(&self, table_name: &str) -> String {
        format!("{}/{}.data", self.base_path, table_name)
    }
}

impl StorageEngine for FileStorage {
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
        
        let file_path = self.get_table_file(table_name);
        let mut file = OpenOptions::new()
            .create()
            .append(true)
            .open(&file_path)
            .map_err(|e| format!("Failed to open file: {}", e))?;
        
        let key_len = key.len() as u32;
        let value_len = value.len() as u32;
        
        file.write_all(&key_len.to_le_bytes()).map_err(|e| format!("Write error: {}", e))?;
        file.write_all(key).map_err(|e| format!("Write error: {}", e))?;
        file.write_all(&value_len.to_le_bytes()).map_err(|e| format!("Write error: {}", e))?;
        file.write_all(value).map_err(|e| format!("Write error: {}", e))?;
        
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
