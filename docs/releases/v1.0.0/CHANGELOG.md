# 变更日志 - v1.0.0

## 新增功能

- #101: 添加 SELECT 语句支持
- #102: 添加 INSERT 语句支持
- #103: 添加 UPDATE 语句支持
- #104: 添加 DELETE 语句支持
- #105: 添加 CREATE TABLE / DROP TABLE 语句
- #106: 添加 B+ 树索引
- #107: 添加 MVCC 事务
- #108: 添加 MySQL 协议兼容服务器
- #109: 添加 REPL 交互式终端
- #110: 添加 QPS 基准测试套件
- #111: 添加 Harness 治理（BP1/BP2 Gate）
- #112: 添加三层模型（提示词-上下文-Harness）实践

## 性能优化

- #201: 优化 SELECT 查询性能（3097 QPS）
- #202: 引入缓冲池减少磁盘 I/O
- #203: B+ 树索引加速等值查询

## Bug 修复

- #301: 修复 INSERT 后索引未及时更新的问题
- #302: 修复 executor 中 clippy::collapsible_match 警告（2 处）
- #303: 修复并发场景下 BufferPool 竞态

## 文档

- #401: 完善 README
- #402: 完善各周实验报告（week-01 ~ week-15）
- #403: 添加 v1.0.0 Release Notes
