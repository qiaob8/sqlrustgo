//! Write-Ahead Log (WAL) for transaction durability
//!
//! Records all transaction operations to disk before applying changes.
//! Ensures durability even after system crash.
//!
//! ## WAL Protocol
//!
//! ```mermaid
//! sequenceDiagram
//!     TxManager->>WAL: BEGIN (tx_id)
//!     WAL-->>Disk: Write WAL record
//!     Disk-->>WAL: Flush
//!     TxManager->>Storage: Modify data
//!     TxManager->>WAL: COMMIT (tx_id)
//!     WAL-->>Disk: Write COMMIT
//! ```
//!
//! ## Log Format
//!
//! Each record is stored as: `[4-byte length][JSON data][newline]`
//!
//! This format allows efficient parsing and supports recovery after crash.
//!
//! ## Record Types
//!
//! - **Begin**: Marks transaction start, captures tx_id
//! - **Commit**: Marks successful completion, data can now be durable
//! - **Rollback**: Marks transaction aborted, changes discarded

use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::sync::Mutex;

/// WAL record types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WalRecord {
    Begin { tx_id: u64 },
    Commit { tx_id: u64 },
    Rollback { tx_id: u64 },
}

/// Write-Ahead Log
#[allow(dead_code)]
pub struct WriteAheadLog {
    file: Mutex<File>,
    path: String,
}

impl WriteAheadLog {
    /// Create or open WAL
    pub fn new(path: &str) -> Result<Self, std::io::Error> {
        let file = OpenOptions::new()
            .read(true)
            .create(true)
            .append(true)
            .open(path)?;

        Ok(Self {
            file: Mutex::new(file),
            path: path.to_string(),
        })
    }

    /// Append a record to the log
    pub fn append(&self, record: &WalRecord) -> Result<(), std::io::Error> {
        let mut file = self.file.lock().expect("Failed to acquire WAL file lock");

        // Serialize to JSON
        let json = serde_json::to_string(record)
            .map_err(|_| std::io::Error::other("serialization failed"))?;

        // Write length prefix + newline
        let len_bytes = (json.len() as u32).to_le_bytes();
        file.write_all(&len_bytes)?;
        file.write_all(json.as_bytes())?;
        file.write_all(b"\n")?;
        file.flush()?;

        Ok(())
    }

    /// Read all records from log
    pub fn read_all(&self) -> Result<Vec<WalRecord>, std::io::Error> {
        let mut file = self.file.lock().expect("Failed to acquire WAL file lock");
        let mut records = Vec::new();

        // Seek to start
        if file.seek(SeekFrom::Start(0)).is_err() {
            return Ok(records);
        }

        loop {
            let mut len_buf = [0u8; 4];
            match file.read_exact(&mut len_buf) {
                Ok(()) => {}
                Err(_) => break, // EOF
            }

            let len = u32::from_le_bytes(len_buf) as usize;
            let mut data = vec![0u8; len];
            if file.read_exact(&mut data).is_err() {
                break;
            }

            // Read newline
            let mut newline = [0u8; 1];
            let _ = file.read(&mut newline);

            if let Ok(record) = serde_json::from_slice(&data) {
                records.push(record);
            }
        }

        Ok(records)
    }

    /// Truncate log (after successful checkpoint)
    pub fn truncate(&self) -> Result<(), std::io::Error> {
        // For Windows compatibility, we need to reopen the file
        drop(self.file.lock());
        
        // Reopen the file with truncate mode
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.path)?;
        
        // Update the file handle
        *self.file.lock().expect("Failed to acquire WAL file lock") = file;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wal_append() {
        let path = std::env::temp_dir().join("wal_test_append.log");
        let path_str = path.to_str().unwrap();
        std::fs::remove_file(path_str).ok();

        let wal = WriteAheadLog::new(path_str).unwrap();
        let record = WalRecord::Begin { tx_id: 1 };
        wal.append(&record).unwrap();

        let records = wal.read_all().unwrap();
        assert_eq!(records.len(), 1);

        std::fs::remove_file(path_str).ok();
    }

    #[test]
    fn test_wal_commit() {
        let path = std::env::temp_dir().join("wal_test_commit.log");
        let path_str = path.to_str().unwrap();
        std::fs::remove_file(path_str).ok();

        let wal = WriteAheadLog::new(path_str).unwrap();

        wal.append(&WalRecord::Begin { tx_id: 1 }).unwrap();
        wal.append(&WalRecord::Commit { tx_id: 1 }).unwrap();

        let records = wal.read_all().unwrap();
        assert_eq!(records.len(), 2);

        std::fs::remove_file(path_str).ok();
    }

    #[test]
    fn test_wal_rollback_record() {
        let path = std::env::temp_dir().join("wal_test_rollback.log");
        let path_str = path.to_str().unwrap();
        std::fs::remove_file(path_str).ok();

        let wal = WriteAheadLog::new(path_str).unwrap();

        wal.append(&WalRecord::Begin { tx_id: 1 }).unwrap();
        wal.append(&WalRecord::Rollback { tx_id: 1 }).unwrap();

        let records = wal.read_all().unwrap();
        assert_eq!(records.len(), 2);

        std::fs::remove_file(path_str).ok();
    }

    #[test]
    fn test_wal_multiple_transactions() {
        let path = std::env::temp_dir().join("wal_test_multi.log");
        let path_str = path.to_str().unwrap();
        std::fs::remove_file(path_str).ok();

        let wal = WriteAheadLog::new(path_str).unwrap();

        // Transaction 1
        wal.append(&WalRecord::Begin { tx_id: 1 }).unwrap();
        wal.append(&WalRecord::Commit { tx_id: 1 }).unwrap();

        // Transaction 2
        wal.append(&WalRecord::Begin { tx_id: 2 }).unwrap();
        wal.append(&WalRecord::Commit { tx_id: 2 }).unwrap();

        let records = wal.read_all().unwrap();
        assert_eq!(records.len(), 4);

        std::fs::remove_file(path_str).ok();
    }

    #[test]
    fn test_wal_truncate() {
        let path = std::env::temp_dir().join("wal_test_truncate.log");
        let path_str = path.to_str().unwrap();
        std::fs::remove_file(path_str).ok();

        let wal = WriteAheadLog::new(path_str).unwrap();

        wal.append(&WalRecord::Begin { tx_id: 1 }).unwrap();
        wal.append(&WalRecord::Commit { tx_id: 1 }).unwrap();

        wal.truncate().unwrap();

        let records = wal.read_all().unwrap();
        assert_eq!(records.len(), 0);

        std::fs::remove_file(path_str).ok();
    }

    // ==================== Additional Coverage Tests ====================

    #[test]
    fn test_wal_empty_file() {
        let path = std::env::temp_dir().join("wal_test_empty.log");
        let path_str = path.to_str().unwrap();
        std::fs::remove_file(path_str).ok();

        // Create empty file
        std::fs::write(path_str, "").ok();

        let wal = WriteAheadLog::new(path_str).unwrap();
        let records = wal.read_all().unwrap();
        assert_eq!(records.len(), 0);

        std::fs::remove_file(path_str).ok();
    }

    #[test]
    fn test_wal_record_variants() {
        // Test WalRecord enum variants
        let begin = WalRecord::Begin { tx_id: 1 };
        let commit = WalRecord::Commit { tx_id: 1 };
        let rollback = WalRecord::Rollback { tx_id: 1 };

        // Debug format should work
        let debug_str = format!("{:?}", begin);
        assert!(debug_str.contains("Begin"));

        let debug_str = format!("{:?}", commit);
        assert!(debug_str.contains("Commit"));

        let debug_str = format!("{:?}", rollback);
        assert!(debug_str.contains("Rollback"));
    }

    #[test]
    fn test_wal_path_access() {
        let path = std::env::temp_dir().join("wal_test_path.log");
        let path_str = path.to_str().unwrap();
        std::fs::remove_file(path_str).ok();

        let wal = WriteAheadLog::new(path_str).unwrap();

        // Access path field (it's public)
        assert!(wal.path.contains("wal_test_path"));

        std::fs::remove_file(path_str).ok();
    }

    #[test]
    fn test_wal_many_records() {
        let path = std::env::temp_dir().join("wal_test_many.log");
        let path_str = path.to_str().unwrap();
        std::fs::remove_file(path_str).ok();

        let wal = WriteAheadLog::new(path_str).unwrap();

        // Append many records
        for i in 1..=100 {
            wal.append(&WalRecord::Begin { tx_id: i }).unwrap();
            wal.append(&WalRecord::Commit { tx_id: i }).unwrap();
        }

        let records = wal.read_all().unwrap();
        assert_eq!(records.len(), 200);

        std::fs::remove_file(path_str).ok();
    }
}
