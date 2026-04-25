pub trait StorageEngine: Send + Sync {
    fn read(&self, table_name: &str, key: &[u8]) -> Result<Option<Vec<u8>>, String>;
    fn write(&mut self, table_name: &str, key: &[u8], value: &[u8]) -> Result<(), String>;
    fn delete(&mut self, table_name: &str, key: &[u8]) -> Result<(), String>;
    fn scan(&self, table_name: &str) -> Result<Box<dyn Iterator<Item = (Vec<u8>, Vec<u8>)> + '_>, String>;
}
