mod crate::results

pub fn query_all_stock_base_info() {

}

pub fn query_stock_base_info(stock_code: &str) {
    let mut conn = super::POOL.get_conn().unwrap();
    let selected_payments = conn
    .query_map(
        "SELECT * from stock_base_info order by trade_date desc limit 1200",
        |(pk_tablemeta, table_name, is_redis)| {
            Payment{pk_tablemeta, table_name, is_redis}
        }
	).unwrap();
	for item in selected_payments {
		println!("{0} {1} {2}", item.pk_tablemeta, item.table_name.unwrap(), item.is_redis.unwrap());
	}
	Ok(cx.string("hello mysql"))
}