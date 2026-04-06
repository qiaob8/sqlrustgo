use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub isbn: String,
    pub title: String,
    pub author: String,
    pub publisher: String,
    pub available: bool,
}

impl Book {
    pub fn new(isbn: String, title: String, author: String, publisher: String) -> Self {
        Self {
            isbn,
            title,
            author,
            publisher,
            available: true,
        }
    }

    pub fn borrow(&mut self) -> Result<(), String> {
        if self.available {
            self.available = false;
            Ok(())
        } else {
            Err(format!("图书《{}》已被借出，无法再次借阅", self.title))
        }
    }

    pub fn return_book(&mut self) -> Result<(), String> {
        if !self.available {
            self.available = true;
            Ok(())
        } else {
            Err(format!("图书《{}》未被借出，无法归还", self.title))
        }
    }

    pub fn is_available(&self) -> bool {
        self.available
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book_creation() {
        let book = Book::new(
            "978-7-111-54742-1".to_string(),
            "Rust程序设计语言".to_string(),
            "Steve Klabnik".to_string(),
            "机械工业出版社".to_string(),
        );
        assert_eq!(book.isbn, "978-7-111-54742-1");
        assert_eq!(book.title, "Rust程序设计语言");
        assert!(book.is_available());
    }

    #[test]
    fn test_book_borrow() {
        let mut book = Book::new(
            "978-7-111-54742-1".to_string(),
            "Rust程序设计语言".to_string(),
            "Steve Klabnik".to_string(),
            "机械工业出版社".to_string(),
        );
        
        assert!(book.borrow().is_ok());
        assert!(!book.is_available());
        
        assert!(book.borrow().is_err());
    }

    #[test]
    fn test_book_return() {
        let mut book = Book::new(
            "978-7-111-54742-1".to_string(),
            "Rust程序设计语言".to_string(),
            "Steve Klabnik".to_string(),
            "机械工业出版社".to_string(),
        );
        
        book.borrow().unwrap();
        assert!(book.return_book().is_ok());
        assert!(book.is_available());
        
        assert!(book.return_book().is_err());
    }
}
