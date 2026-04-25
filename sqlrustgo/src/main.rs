use sqlrustgo::SqlRustGo;

fn main() {
    println!("SQLRustGo 1.0 Database System");
    println!("=============================");
    
    // 创建数据库实例
    let mut db = SqlRustGo::new();
    
    // 示例SQL语句
    let sql_statements = [
        "CREATE TABLE users (id INT, name VARCHAR(255), age INT)",
        "INSERT INTO users VALUES (1, 'Alice', 25)",
        "INSERT INTO users VALUES (2, 'Bob', 30)",
        "SELECT * FROM users",
        "UPDATE users SET age = 26 WHERE id = 1",
        "DELETE FROM users WHERE id = 2",
        "SELECT * FROM users",
    ];
    
    // 执行SQL语句
    for sql in &sql_statements {
        println!("\nExecuting: {}", sql);
        match db.execute(sql) {
            Ok(result) => println!("Result: {}", result),
            Err(error) => println!("Error: {}", error),
        }
    }
    
    println!("\nSQLRustGo 1.0 Demo Complete!");
}
