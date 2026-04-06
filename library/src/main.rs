mod models;
mod services;

use chrono::NaiveDate;
use models::book::Book;
use models::fine_calculator::{FineCalculator, StudentFineCalculator, TeacherFineCalculator};
use models::user::{Student, Teacher, User};
use services::borrow_service::BorrowService;

fn main() {
    println!("=== 高校图书借阅系统演示 ===\n");

    let mut service = BorrowService::new();

    let student = Student::new("S001".to_string(), "张三".to_string());
    let teacher = Teacher::new(
        "T001".to_string(),
        "李教授".to_string(),
        "计算机学院".to_string(),
    );

    println!("--- 用户信息 ---");
    print_user_info(&student);
    print_user_info(&teacher);
    println!();

    let mut book1 = Book::new(
        "978-7-111-54742-1".to_string(),
        "Rust程序设计语言".to_string(),
        "Steve Klabnik".to_string(),
        "机械工业出版社".to_string(),
    );
    let mut book2 = Book::new(
        "978-7-115-47833-9".to_string(),
        "深入理解计算机系统".to_string(),
        "Randal E.Bryant".to_string(),
        "人民邮电出版社".to_string(),
    );

    println!("--- 图书信息 ---");
    print_book_info(&book1);
    print_book_info(&book2);
    println!();

    let borrow_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();

    println!("--- 借阅操作 ---");
    let record1 = service
        .borrow_book("BR001".to_string(), &student, &mut book1, borrow_date, 30)
        .unwrap();
    println!(
        "学生 {} 借阅《{}》，应还日期: {}",
        record1.user_name, record1.book_title, record1.due_date
    );

    let record2 = service
        .borrow_book("BR002".to_string(), &teacher, &mut book2, borrow_date, 60)
        .unwrap();
    println!(
        "教师 {} 借阅《{}》，应还日期: {}",
        record2.user_name, record2.book_title, record2.due_date
    );
    println!();

    let check_date = NaiveDate::from_ymd_opt(2024, 2, 10).unwrap();
    println!("--- 逾期检查 (当前日期: {}) ---", check_date);

    let overdue_records = service.get_overdue_records(check_date);
    if overdue_records.is_empty() {
        println!("暂无逾期记录");
    } else {
        for record in overdue_records {
            println!(
                "逾期: {} 借阅的《{}》，逾期 {} 天",
                record.user_name,
                record.book_title,
                record.overdue_days(check_date)
            );
        }
    }
    println!();

    println!("--- 罚款计算 ---");
    let student_calculator = StudentFineCalculator::default();
    let teacher_calculator = TeacherFineCalculator::default();

    let student_fine = service.calculate_total_fine("S001", check_date, &student_calculator);
    let teacher_fine = service.calculate_total_fine("T001", check_date, &teacher_calculator);

    println!(
        "学生 {} 罚款: ¥{:.2} (费率: ¥{:.2}/天)",
        student.name(),
        student_fine,
        student_calculator.fine_rate()
    );
    println!(
        "教师 {} 罚款: ¥{:.2} (费率: ¥{:.2}/天)",
        teacher.name(),
        teacher_fine,
        teacher_calculator.fine_rate()
    );
    println!();

    println!("--- 归还操作 ---");
    let return_date = NaiveDate::from_ymd_opt(2024, 2, 10).unwrap();
    let returned = service
        .return_book("BR001", &mut book1, return_date)
        .unwrap();
    println!(
        "{} 已归还《{}》，{}",
        returned.user_name,
        returned.book_title,
        if returned.is_overdue(return_date) {
            format!("逾期 {} 天", returned.overdue_days(return_date))
        } else {
            "按时归还".to_string()
        }
    );
    println!("图书《{}》当前状态: {}", book1.title, if book1.is_available() { "可借" } else { "已借出" });
    println!();

    println!("--- 借阅历史 ---");
    for record in service.all_records() {
        println!(
            "[{}] {} ({}) - 《{}》借阅日期: {}, 状态: {}",
            record.id,
            record.user_name,
            record.user_type,
            record.book_title,
            record.borrow_date,
            if record.is_returned() {
                format!("已归还 ({})", record.return_date.unwrap())
            } else {
                "借阅中".to_string()
            }
        );
    }
}

fn print_user_info(user: &dyn User) {
    println!(
        "{} {} (ID: {}), 借阅上限: {} 本",
        user.user_type(),
        user.name(),
        user.id(),
        user.borrow_limit()
    );
}

fn print_book_info(book: &Book) {
    println!(
        "《{}》- {} ({}), ISBN: {}, 状态: {}",
        book.title,
        book.author,
        book.publisher,
        book.isbn,
        if book.is_available() { "可借" } else { "已借出" }
    );
}
