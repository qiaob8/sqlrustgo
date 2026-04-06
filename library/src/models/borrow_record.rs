use chrono::{NaiveDate, Duration};
use serde::{Deserialize, Serialize};

use super::fine_calculator::FineCalculator;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorrowRecord {
    pub id: String,
    pub user_id: String,
    pub user_name: String,
    pub user_type: String,
    pub book_isbn: String,
    pub book_title: String,
    pub borrow_date: NaiveDate,
    pub due_date: NaiveDate,
    pub return_date: Option<NaiveDate>,
}

impl BorrowRecord {
    pub fn new(
        id: String,
        user_id: String,
        user_name: String,
        user_type: String,
        book_isbn: String,
        book_title: String,
        borrow_date: NaiveDate,
        borrow_days: u32,
    ) -> Self {
        let due_date = borrow_date + Duration::days(borrow_days as i64);
        Self {
            id,
            user_id,
            user_name,
            user_type,
            book_isbn,
            book_title,
            borrow_date,
            due_date,
            return_date: None,
        }
    }

    pub fn is_overdue(&self, current_date: NaiveDate) -> bool {
        if let Some(returned) = self.return_date {
            returned > self.due_date
        } else {
            current_date > self.due_date
        }
    }

    pub fn overdue_days(&self, current_date: NaiveDate) -> i64 {
        if self.is_overdue(current_date) {
            let check_date = self.return_date.unwrap_or(current_date);
            (check_date - self.due_date).num_days()
        } else {
            0
        }
    }

    pub fn calculate_fine(&self, current_date: NaiveDate, calculator: &dyn FineCalculator) -> f64 {
        calculator.calculate(self.overdue_days(current_date))
    }

    pub fn return_book(&mut self, return_date: NaiveDate) -> Result<(), String> {
        if self.return_date.is_some() {
            Err("该借阅记录已完成归还".to_string())
        } else {
            self.return_date = Some(return_date);
            Ok(())
        }
    }

    pub fn is_returned(&self) -> bool {
        self.return_date.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_record() -> BorrowRecord {
        BorrowRecord::new(
            "BR001".to_string(),
            "S001".to_string(),
            "张三".to_string(),
            "学生".to_string(),
            "978-7-111-54742-1".to_string(),
            "Rust程序设计语言".to_string(),
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            30,
        )
    }

    #[test]
    fn test_borrow_record_creation() {
        let record = create_test_record();
        assert_eq!(record.id, "BR001");
        assert_eq!(record.borrow_date, NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
        assert_eq!(record.due_date, NaiveDate::from_ymd_opt(2024, 1, 31).unwrap());
        assert!(!record.is_returned());
    }

    #[test]
    fn test_is_overdue() {
        let record = create_test_record();
        
        assert!(!record.is_overdue(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()));
        assert!(!record.is_overdue(NaiveDate::from_ymd_opt(2024, 1, 31).unwrap()));
        assert!(record.is_overdue(NaiveDate::from_ymd_opt(2024, 2, 1).unwrap()));
    }

    #[test]
    fn test_overdue_days() {
        let record = create_test_record();
        
        assert_eq!(record.overdue_days(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()), 0);
        assert_eq!(record.overdue_days(NaiveDate::from_ymd_opt(2024, 2, 5).unwrap()), 5);
    }

    #[test]
    fn test_return_book() {
        let mut record = create_test_record();
        
        assert!(record.return_book(NaiveDate::from_ymd_opt(2024, 1, 20).unwrap()).is_ok());
        assert!(record.is_returned());
        assert!(record.return_book(NaiveDate::from_ymd_opt(2024, 1, 25).unwrap()).is_err());
    }
}
