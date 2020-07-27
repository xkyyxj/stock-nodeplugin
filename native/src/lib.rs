use neon::prelude::*;
use mysql::*;
use mysql::prelude::*;
extern crate ocl;
use once_cell::sync::Lazy;

const url: &str = "mysql://wqch:123456@localhost:3306/stock";

struct Test {
	value: Option<String>,
}

impl Test {
	pub fn new() -> Self {
		Test { value: Some(String::from("what")) }
	}
}

impl Drop for Test {
	fn drop(&mut self) {
        println!("Dropping Test with data `{}`!", self.data);
    }
}

/**
 * 全局静态的数据库连接池
 * TODO: N-API下、多模块导入如何适配？
 */
static POOL: Lazy<Pool> = Lazy::new(|| {
    Pool::new(url).unwrap()
});

static REDIS: Lazy<redis::Client> = Lazy::new(|| {
	redis::Client::open("redis://127.0.0.1/").unwrap()
});

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    pk_tablemeta: i32,
    table_name: Option<String>,
    is_redis: Option<String>,
}

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
	let temp_val = vec![1,2,3];
	for i in temp_val {
		println!("{:?}", i);
	}
	
	let all_platforms = ocl::Platform::list();
	for platform in all_platforms {
		println!("{0}", platform.name().unwrap());
	}
    Ok(cx.string("hello node"))
}

fn calculate_max_win(mut cx: FunctionContext) -> JsResult<JsString> {
	let mut conn = POOL.get_conn().unwrap();
	let selected_payments = conn
    .query_map(
        "SELECT pk_tablemeta, table_name, is_redis from table_meta",
        |(pk_tablemeta, table_name, is_redis)| {
            Payment{pk_tablemeta, table_name, is_redis}
        }
	).unwrap();
	for item in selected_payments {
		println!("{0} {1} {2}", item.pk_tablemeta, item.table_name.unwrap(), item.is_redis.unwrap());
	}
	Ok(cx.string("hello mysql"))
}

register_module!(mut cx, {
	cx.export_function("hello", hello)?;
	cx.export_function("calculate_max_win", calculate_max_win)?;
	Ok(())
});
