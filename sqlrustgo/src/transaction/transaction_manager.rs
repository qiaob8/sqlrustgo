use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionState {
    Active,
    Committed,
    Aborted,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: u64,
    pub state: TransactionState,
}

pub struct TransactionManager {
    next_id: u64,
    transactions: HashMap<u64, Transaction>,
    active_transactions: Vec<u64>,
}

impl TransactionManager {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            transactions: HashMap::new(),
            active_transactions: Vec::new(),
        }
    }
    
    pub fn begin(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        
        let transaction = Transaction {
            id,
            state: TransactionState::Active,
        };
        
        self.transactions.insert(id, transaction);
        self.active_transactions.push(id);
        
        id
    }
    
    pub fn commit(&mut self, tx_id: u64) -> Result<(), String> {
        match self.transactions.get_mut(&tx_id) {
            Some(tx) => {
                if tx.state != TransactionState::Active {
                    return Err(format!("Transaction {} is not active", tx_id));
                }
                tx.state = TransactionState::Committed;
                self.active_transactions.retain(|&id| id != tx_id);
                Ok(())
            }
            None => Err(format!("Transaction {} not found", tx_id)),
        }
    }
    
    pub fn rollback(&mut self, tx_id: u64) -> Result<(), String> {
        match self.transactions.get_mut(&tx_id) {
            Some(tx) => {
                if tx.state != TransactionState::Active {
                    return Err(format!("Transaction {} is not active", tx_id));
                }
                tx.state = TransactionState::Aborted;
                self.active_transactions.retain(|&id| id != tx_id);
                Ok(())
            }
            None => Err(format!("Transaction {} not found", tx_id)),
        }
    }
    
    pub fn get_transaction(&self, tx_id: u64) -> Option<&Transaction> {
        self.transactions.get(&tx_id)
    }
}
