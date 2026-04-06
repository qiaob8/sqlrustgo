use crate::models::book::Book;
use crate::models::borrow_record::BorrowRecord;
use crate::models::fine_calculator::FineCalculator;
use crate::models::user::User;
use chrono::NaiveDate;

pub struct BorrowService {
    records: Vec<BorrowRecord>,
}

impl BorrowService {
    pub fn new() -> Self {
        Self { records: Vec::new() }
    }

    pub fn borrow_book(
        &mut self,
        record_id: String,
        user: &dyn User,
        book: &mut Book,
        borrow_date: NaiveDate,
        borrow_days: u32,
    ) -> Result<&BorrowRecord, String> {
        book.borrow()?;

        let record = BorrowRecord::new(
            record_id,
            user.id().to_string(),
            user.name().to_string(),
            user.user_type().to_string(),
            book.isbn.clone(),
            book.title.clone(),
            borrow_date,
            borrow_days,
        );

        self.records.push(record);
        Ok(self.records.last().unwrap())
    }

    pub fn return_book(
        &mut self,
        record_id: &str,
        book: &mut Book,
        return_date: NaiveDate,
    ) -> Result<&BorrowRecord, String> {
        let record = self
            .records
            .iter_mut()
            .find(|r| r.id == record_id)
            .ok_or("借阅记录不存在")?;

        record.return_book(return_date)?;
        book.return_book()?;

        Ok(record)
    }

    pub fn get_user_records(&self, user_id: &str) -> Vec<&BorrowRecord> {
        self.records
            .iter()
            .filter(|r| r.user_id == user_id)
            .collect()
    }

    pub fn get_overdue_records(&self, current_date: NaiveDate) -> Vec<&BorrowRecord> {
        self.records
            .iter()
            .filter(|r| r.is_overdue(current_date) && !r.is_returned())
            .collect()
    }

    pub fn calculate_total_fine(
        &self,
        user_id: &str,
        current_date: NaiveDate,
        calculator: &dyn FineCalculator,
    ) -> f64 {
        self.records
            .iter()
            .filter(|r| r.user_id == user_id)
            .map(|r| r.calculate_fine(current_date, calculator))
            .sum()
    }

    pub fn all_records(&self) -> &[BorrowRecord] {
        &self.records
    }
}

impl Default for BorrowService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::fine_calculator::DefaultFineCalculator;
    use crate::models::user::Student;

    #[test]
    fn test_borrow_service() {
        let mut service = BorrowService::new();
        let student = Student::new("S001".to_string(), "张三".to_string());
        let mut book = Book::new(
            "978-7-111-54742-1".to_string(),
            "Rust程序设计语言".to_string(),
            "Steve Klabnik".to_string(),
            "机械工业出版社".to_string(),
        );

        let borrow_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        
        let record = service
            .borrow_book(
                "BR001".to_string(),
                &student,
                &mut book,
                borrow_date,
                30,
            )
            .unwrap();

        assert_eq!(record.user_id, "S001");
        assert_eq!(record.book_isbn, "978-7-111-54742-1");
        assert!(!book.is_available());

        let return_date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        let returned = service
            .return_book("BR001", &mut book, return_date)
            .unwrap();

        assert!(returned.is_returned());
        assert!(book.is_available());
    }

    #[test]
    fn test_overdue_calculation() {
        let mut service = BorrowService::new();
        let student = Student::new("S001".to_string(), "张三".to_string());
        let mut book = Book::new(
            "978-7-111-54742-1".to_string(),
            "Rust程序设计语言".to_string(),
            "Steve Klabnik".to_string(),
            "机械工业出版社".to_string(),
        );

        let borrow_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        
        service
            .borrow_book(
                "BR001".to_string(),
                &student,
                &mut book,
                borrow_date,
                30,
            )
            .unwrap();

        let current_date = NaiveDate::from_ymd_opt(2024, 2, 10).unwrap();
        let overdue = service.get_overdue_records(current_date);
        assert_eq!(overdue.len(), 1);

        let calculator = DefaultFineCalculator::default();
        let fine = service.calculate_total_fine("S001", current_date, &calculator);
        assert_eq!(fine, 2.0);
    }
}
