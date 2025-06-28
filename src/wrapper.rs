use rbatis::RBatis;
use rbatis::Error;
use serde::Serialize;

// 添加分页结果结构体
#[derive(Debug, Serialize)]
pub struct Page<T> {
    pub records: Vec<T>,         // 数据列表
    pub total: u64,             // 总记录数
    pub page_no: u64,           // 当前页码
    pub page_size: u64,         // 每页大小
    pub pages: u64,             // 总页数
    pub has_next: bool,         // 是否有下一页
}

impl<T> Page<T> {
    pub fn new(records: Vec<T>, total: u64, page_no: u64, page_size: u64) -> Self {
        let pages = (total + page_size - 1) / page_size;
        let has_next = page_no < pages;
        
        Self {
            records,
            total,
            page_no,
            page_size,
            pages,
            has_next,
        }
    }
}

/// like mybatis plus
/// for example:
/// ```
/// let count = QueryWrapper::new()
///     .custom_sql("select count(*) from member")
///     .get_one::<u64>(&RB, "")
///     .await?;
/// println!("count: {:?}", count);

/// #[derive(serde::Deserialize, serde::Serialize, Debug)]
/// struct Member {
///     id: u64,
///     email: Option<String>
/// }

/// let member = QueryWrapper::new()
///     .eq("id", 7386)
///     .get_one::<Member>(&RB, "member")
///     .await?;
/// println!("member: {:?}", member);

/// Ok(Json(json!({
///     "code": 0,
///     "data": member,
///     "count": count,
/// })))
/// ```
#[derive(Default, Debug, Clone)]
pub struct QueryWrapper {
    where_conditions: Vec<String>,
    order_by: Vec<String>,
    select_columns: Vec<String>,
    limit: Option<u64>,
    offset: Option<u64>,
    custom_sql: Option<String>,    // 添加自定义SQL支持
    join_conditions: Vec<String>,  // 添加JOIN条件支持
}

impl QueryWrapper {
    pub fn new() -> Self {
        Self::default()
    }

    // 等于条件
    pub fn eq<T: ToString>(mut self, column: &str, value: T) -> Self {
        self.where_conditions.push(format!("{} = '{}'", column, value.to_string()));
        self
    }

    // 不等于条件
    pub fn ne<T: ToString>(mut self, column: &str, value: T) -> Self {
        self.where_conditions.push(format!("{} != '{}'", column, value.to_string()));
        self
    }

    // 大于条件
    pub fn gt<T: ToString>(mut self, column: &str, value: T) -> Self {
        self.where_conditions.push(format!("{} > '{}'", column, value.to_string()));
        self
    }

    // 小于条件
    pub fn lt<T: ToString>(mut self, column: &str, value: T) -> Self {
        self.where_conditions.push(format!("{} < '{}'", column, value.to_string()));
        self
    }

    // LIKE 条件
    pub fn like(mut self, column: &str, value: &str) -> Self {
        self.where_conditions.push(format!("{} LIKE '%{}%'", column, value));
        self
    }

    // 指定查询列
    pub fn select(mut self, columns: Vec<&str>) -> Self {
        self.select_columns = columns.into_iter().map(String::from).collect();
        self
    }

    // 排序
    pub fn order_by(mut self, column: &str, asc: bool) -> Self {
        let order = if asc { "ASC" } else { "DESC" };
        self.order_by.push(format!("{} {}", column, order));
        self
    }

    // 修改 limit 方法为引用
    pub fn limit(&mut self, limit: u64) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    // 修改 offset 方法为引用
    pub fn offset(&mut self, offset: u64) -> &mut Self {
        self.offset = Some(offset);
        self
    }

    // 添加自定义SQL方法
    pub fn custom_sql(mut self, sql: &str) -> Self {
        self.custom_sql = Some(sql.to_string());
        self
    }

    // 添加 INNER JOIN
    pub fn inner_join(mut self, table: &str, on_condition: &str) -> Self {
        self.join_conditions.push(format!("INNER JOIN {} ON {}", table, on_condition));
        self
    }

    // 添加 LEFT JOIN
    pub fn left_join(mut self, table: &str, on_condition: &str) -> Self {
        self.join_conditions.push(format!("LEFT JOIN {} ON {}", table, on_condition));
        self
    }

    // 添加 RIGHT JOIN
    pub fn right_join(mut self, table: &str, on_condition: &str) -> Self {
        self.join_conditions.push(format!("RIGHT JOIN {} ON {}", table, on_condition));
        self
    }

    // 修改构建SQL语句方法
    pub fn build_sql(&self, table_name: &str) -> String {
        // 如果有自定义SQL，直接使用它
        if let Some(custom_sql) = &self.custom_sql {
            let mut sql = custom_sql.clone();
            
            // 添加WHERE条件
            if !self.where_conditions.is_empty() {
                if !sql.to_uppercase().contains("WHERE") {
                    sql.push_str(" WHERE ");
                } else {
                    sql.push_str(" AND ");
                }
                sql.push_str(&self.where_conditions.join(" AND "));
            }

            // 添加排序
            if !self.order_by.is_empty() {
                sql.push_str(" ORDER BY ");
                sql.push_str(&self.order_by.join(", "));
            }

            // 添加分页
            if let Some(limit) = self.limit {
                sql.push_str(&format!(" LIMIT {}", limit));
            }
            if let Some(offset) = self.offset {
                sql.push_str(&format!(" OFFSET {}", offset));
            }

            return sql;
        }

        // 常规SQL构建
        let select = if self.select_columns.is_empty() {
            "*".to_string()
        } else {
            self.select_columns.join(", ")
        };

        let mut sql = format!("SELECT {} FROM {}", select, table_name);

        // 添加JOIN条件
        if !self.join_conditions.is_empty() {
            sql.push_str(" ");
            sql.push_str(&self.join_conditions.join(" "));
        }

        if !self.where_conditions.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&self.where_conditions.join(" AND "));
        }

        if !self.order_by.is_empty() {
            sql.push_str(" ORDER BY ");
            sql.push_str(&self.order_by.join(", "));
        }

        if let Some(limit) = self.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = self.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        sql
    }

    // 执行查询
    pub async fn query<T>(&self, rb: &RBatis, table_name: &str) -> Result<Vec<T>, Error>
    where
        T: Serialize + for<'de> serde::Deserialize<'de>,
    {
        let sql = self.build_sql(table_name);
        rb.query_decode(&sql, vec![]).await
    }

    // 执行查询
    pub async fn get_one<T>(&self, rb: &RBatis, table_name: &str) -> Result<Option<T>, Error>
    where
        T: Serialize + for<'de> serde::Deserialize<'de>,
    {
        let sql = self.build_sql(table_name);
        rb.query_decode::<Option<T>>(&sql, vec![]).await
    }

    // 执行删除
    pub async fn delete(self, rb: &RBatis, table_name: &str) -> Result<u64, Error> {
        let delete_sql = format!("delete from {}", table_name);
        let sql = self.custom_sql(&delete_sql)
            .build_sql(table_name);
        Ok(rb.exec(&sql, vec![]).await?.rows_affected)
    }

    // 修改分页方法
    pub async fn page<T>(&self, rb: &RBatis, table_name: &str, page_no: u64, page_size: u64) -> Result<Page<T>, Error>
    where
        T: Serialize + for<'de> serde::Deserialize<'de>,
    {
        // 1. 先查询总记录数
        let count_sql = self.build_count_sql(table_name);
        let total: u64 = rb.query_decode(&count_sql, vec![]).await?;

        // 2. 如果有数据，再查询分页数据
        if total > 0 {
            // 设置分页参数
            let offset = (page_no - 1) * page_size;
            let mut wrapper = self.clone();
            wrapper.limit(page_size);  // 现在这些方法返回 &mut Self
            wrapper.offset(offset);    // 可以分开调用
            
            // 查询分页数据
            let records: Vec<T> = wrapper.query(rb, table_name).await?;
            
            Ok(Page::new(records, total, page_no, page_size))
        } else {
            // 没有数据时返回空页
            Ok(Page::new(vec![], 0, page_no, page_size))
        }
    }

    // 修改构建统计SQL方法
    fn build_count_sql(&self, table_name: &str) -> String {
        if let Some(custom_sql) = &self.custom_sql {
            // 将 WHERE 条件放入子查询内部
            let mut inner_sql = custom_sql.clone();
            
            if !self.where_conditions.is_empty() {
                if !inner_sql.to_uppercase().contains("WHERE") {
                    inner_sql.push_str(" WHERE ");
                } else {
                    inner_sql.push_str(" AND ");
                }
                inner_sql.push_str(&self.where_conditions.join(" AND "));
            }

            // 包装成计数查询
            format!("SELECT COUNT(*) FROM ({}) as t", inner_sql)
        } else {
            let mut sql = format!("SELECT COUNT(*) FROM {}", table_name);

            // 添加JOIN条件
            if !self.join_conditions.is_empty() {
                sql.push_str(" ");
                sql.push_str(&self.join_conditions.join(" "));
            }

            if !self.where_conditions.is_empty() {
                sql.push_str(" WHERE ");
                sql.push_str(&self.where_conditions.join(" AND "));
            }

            sql
        }
    }
}