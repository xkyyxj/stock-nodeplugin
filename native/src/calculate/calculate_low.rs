use crate::sql;

pub fn calculate_in_low() {
    let columns = vec!["ts_code"];
    let query_list_fut = sql::query_stock_list(&columns, "");
    let thread_pool = crate::THREAD_POOL.get().unwrap();
    thread_pool.spawn(query_list_fut);
    for item in columns {

    }
}