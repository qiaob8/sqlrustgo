AI 安全审查请求：

审查 src/storage/file_storage.rs 第 250-270 行的索引构建代码：

```rust
for row in rows.iter() {
    if let Some(value) = row.get(column_index) {
        if let Value::Integer(key) = value {           // <-- clippy 警告 1
            index.insert(*key, row_id as u32);
        }
    }
    row_id += 1;
}
```

请检查：
1. SQL 注入风险
2. 缓冲区溢出风险
3. 敏感信息泄露
4. 不安全的加密使用
5. 其他安全问题

重点关注：
- row.get() 返回 Option<Value>，value 类型为 i64，as u32 是否会溢出
- row_id 自增无上限，长期运行是否会导致溢出
- index.insert 在并发场景下是否有竞态
