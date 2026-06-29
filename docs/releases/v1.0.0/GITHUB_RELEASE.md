# SQLRustGo v1.0.0 - 首个稳定版本

SQLRustGo v1.0.0 是首个**稳定版本**，标志着项目从开发阶段进入生产就绪状态。

## 亮点

- **SQL-92 子集**：SELECT / INSERT / UPDATE / DELETE / CREATE TABLE / DROP TABLE
- **存储引擎**：页式存储 + 缓冲池 + B+ 树索引 + WAL
- **事务**：MVCC + 快照隔离
- **网络**：TCP 服务器 + MySQL 协议兼容 + REPL

## QPS 基准

| 操作 | QPS |
|------|-----|
| SELECT | 3097 |
| DELETE | 146 |
| UPDATE | 83 |
| INSERT | 88 |

## 完整发布说明

详见 [RELEASE_NOTES.md](./RELEASE_NOTES.md) 和 [CHANGELOG.md](./CHANGELOG.md)。

## 安装

```bash
git clone https://github.com/qiaob8/sqlrustgo.git
cd sqlrustgo
cargo test
```

## 致谢

感谢李莹老师的指导，以及实验小组成员的协作。
