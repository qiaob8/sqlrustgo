-- SQLRustGo 测试脚本

-- 创建表
CREATE TABLE users (
    id INT,
    name VARCHAR(255),
    age INT
);

-- 插入数据
INSERT INTO users VALUES (1, 'Alice', 25);
INSERT INTO users VALUES (2, 'Bob', 30);
INSERT INTO users VALUES (3, 'Charlie', 35);

-- 查询数据
SELECT * FROM users;
SELECT id, name FROM users WHERE age > 25;

-- 更新数据
UPDATE users SET age = 26 WHERE id = 1;

-- 删除数据
DELETE FROM users WHERE id = 3;

-- 再次查询
SELECT * FROM users;