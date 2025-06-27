# rbatis-wrapper

🚀 一个基于 [rbatis](https://github.com/rbatis/rbatis) 的现代化查询构建器，类似于 MyBatis Plus 的链式查询风格。

[![Crates.io](https://img.shields.io/crates/v/rbatis-wrapper.svg)](https://crates.io/crates/rbatis-wrapper)
[![Documentation](https://docs.rs/rbatis-wrapper/badge.svg)](https://docs.rs/rbatis-wrapper)
[![License](https://img.shields.io/crates/l/rbatis-wrapper.svg)](LICENSE)

## ✨ 特性

- 🔗 **链式调用**: 类似 MyBatis Plus 的查询构建器风格
- 📄 **分页支持**: 内置分页功能，支持总数统计
- 🛠 **自定义SQL**: 支持复杂的自定义 SQL 查询
- 🔄 **JOIN查询**: 支持 INNER JOIN、LEFT JOIN、RIGHT JOIN
- 🎯 **类型安全**: 基于泛型的类型安全查询
- ⚡ **异步支持**: 完全支持 Rust async/await

## 📦 安装

在您的 `Cargo.toml` 中添加以下依赖：

```toml
[dependencies]
rbatis-wrapper = "0.1.0"
rbatis = { version = "4.6", features = ["debug_mode"] }
serde = { version = "1.0", features = ["derive"] }
```

## 🚀 快速开始

### 基础设置

```rust
use rbatis::RBatis;
use rbatis_wrapper::QueryWrapper;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    email: Option<String>,
    age: u32,
}

// 初始化 RBatis 实例
static RB: Lazy<RBatis> = Lazy::new(|| RBatis::new());
```

### 基础查询

```rust
// 查询单个记录
let user = QueryWrapper::new()
    .eq("id", 1)
    .get_one::<User>(&RB, "users")
    .await?;

// 查询多个记录
let users = QueryWrapper::new()
    .gt("age", 18)
    .like("name", "张")
    .order_by("age", true) // true 为升序
    .query::<User>(&RB, "users")
    .await?;
```

### 条件查询

```rust
let users = QueryWrapper::new()
    .eq("status", 1)                    // 等于
    .ne("deleted", 1)                   // 不等于  
    .gt("age", 18)                      // 大于
    .lt("age", 60)                      // 小于
    .like("name", "张")                 // LIKE 模糊查询
    .query::<User>(&RB, "users")
    .await?;
```

### 分页查询

```rust
let page_result = QueryWrapper::new()
    .eq("status", 1)
    .order_by("created_at", false) // 按创建时间降序
    .page::<User>(&RB, "users", 1, 10) // 第1页，每页10条
    .await?;

println!("总记录数: {}", page_result.total);
println!("当前页: {}", page_result.page_no);
println!("总页数: {}", page_result.pages);
println!("是否有下一页: {}", page_result.has_next);
```

### 指定查询字段

```rust
let users = QueryWrapper::new()
    .select(vec!["id", "name", "email"])
    .eq("status", 1)
    .query::<User>(&RB, "users")
    .await?;
```

### JOIN 查询

```rust
let results = QueryWrapper::new()
    .inner_join("profiles p", "u.id = p.user_id")
    .left_join("orders o", "u.id = o.user_id")
    .eq("u.status", 1)
    .query::<User>(&RB, "users u")
    .await?;
```

### 自定义 SQL

```rust
// 自定义查询
let users = QueryWrapper::new()
    .custom_sql("SELECT * FROM users WHERE age BETWEEN 18 AND 65")
    .eq("status", 1) // 会自动添加到 WHERE 条件
    .query::<User>(&RB, "")
    .await?;

// 统计查询
let count = QueryWrapper::new()
    .custom_sql("SELECT COUNT(*) FROM users")
    .eq("status", 1)
    .get_one::<u64>(&RB, "")
    .await?;
```

### 复杂查询示例

```rust
// 复合条件查询
let mut wrapper = QueryWrapper::new();
wrapper
    .limit(20)
    .offset(40);

let users = wrapper
    .eq("department", "技术部")
    .gt("salary", 8000)
    .order_by("hire_date", false)
    .query::<User>(&RB, "employees")
    .await?;
```

## 📖 API 文档

### QueryWrapper 方法

| 方法 | 描述 | 示例 |
|------|------|------|
| `new()` | 创建新的查询构建器 | `QueryWrapper::new()` |
| `eq(column, value)` | 等于条件 | `.eq("id", 1)` |
| `ne(column, value)` | 不等于条件 | `.ne("status", 0)` |
| `gt(column, value)` | 大于条件 | `.gt("age", 18)` |
| `lt(column, value)` | 小于条件 | `.lt("price", 100)` |
| `like(column, value)` | LIKE 模糊查询 | `.like("name", "张")` |
| `select(columns)` | 指定查询字段 | `.select(vec!["id", "name"])` |
| `order_by(column, asc)` | 排序 | `.order_by("created_at", false)` |
| `limit(size)` | 限制记录数 | `.limit(10)` |
| `offset(size)` | 偏移量 | `.offset(20)` |
| `inner_join(table, on)` | 内连接 | `.inner_join("profiles", "users.id = profiles.user_id")` |
| `left_join(table, on)` | 左连接 | `.left_join("orders", "users.id = orders.user_id")` |
| `right_join(table, on)` | 右连接 | `.right_join("departments", "users.dept_id = departments.id")` |
| `custom_sql(sql)` | 自定义SQL | `.custom_sql("SELECT * FROM complex_view")` |
| `query<T>(rb, table)` | 执行查询 | `.query::<User>(&RB, "users")` |
| `get_one<T>(rb, table)` | 查询单条记录 | `.get_one::<User>(&RB, "users")` |
| `page<T>(rb, table, page_no, page_size)` | 分页查询 | `.page::<User>(&RB, "users", 1, 10)` |

### Page 结构体

```rust
pub struct Page<T> {
    pub records: Vec<T>,      // 数据列表
    pub total: u64,          // 总记录数
    pub page_no: u64,        // 当前页码
    pub page_size: u64,      // 每页大小
    pub pages: u64,          // 总页数
    pub has_next: bool,      // 是否有下一页
}
```

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

1. Fork 本项目
2. 创建您的特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交您的修改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启一个 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- [rbatis](https://github.com/rbatis/rbatis) - 优秀的 Rust ORM 框架
- [MyBatis Plus](https://baomidou.com/) - API 设计灵感来源 