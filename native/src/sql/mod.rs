use std::ops::Add;
use sqlx::Error;

pub fn query_all_stock_base_info() {

}

/// 查询所有的股票列表
/// #columns : 查询的列
/// #where_part : 过滤条件，需自带“where”，笑
pub async fn query_stock_list(columns: &Vec<&str>, where_part: &str) -> Result<Vec<DB::Row>, Error> {
    let mut query_sql = String::new();
    query_sql.add("select ");
    if columns.is_empty() {
        query_sql.add("* from stock_list ");
    }
    else {
        for item in columns {
            query_sql.add(item).add(",");
        }
        // 弹出最后一个","
        query_sql.pop();
        query_sql.add(" from stock_list ");
    }

    if !where_part.is_empty() {
        query_sql.add(where_part);
    }

    let conn = super::MYSQL_POOL.get().unwrap();
    sqlx::query(query_sql.as_str()).fetch_all(conn).await
}

pub async fn query_stock_base_info(stock_code: &str, where_part: &str) -> Result<Vec<DB::Row>, Error> {
    let query_sql = String::new();
    query_sql.add("select * from stock_base_info where ts_code='");
    query_sql.add(stock_code).add("'");
    if !where_part.is_empty() {
        query_sql.add(where_part);
    }
    let conn = super::MYSQL_POOL.get().unwrap();
    sqlx::query(query_sql.as_str()).fetch_all(conn).await
}