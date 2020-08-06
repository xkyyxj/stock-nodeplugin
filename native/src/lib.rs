mod config;
mod calculate;
mod results;
mod sql;

extern crate ocl;
use neon::prelude::*;
use once_cell::sync::OnceCell;
use futures::executor::{ ThreadPool, ThreadPoolBuilder};
use futures::channel::mpsc::{self, Receiver, Sender};
use sqlx::Pool;
use redis::{Client, RedisError};

static THREAD_POOL: OnceCell<ThreadPool> = OnceCell::new();

static MYSQL_POOL: OnceCell<Pool<MySql>> = OnceCell::new();

static REDIS_POOL: OnceCell<Client> = OnceCell::new();

fn init(mut cx: FunctionContext) -> JsResult<JsBoolean>  {
	let mut final_rst = false;

	let mysql_info = cx.argument::<JsString>(0)?.value();
	let redis_info = cx.argument::<JsString>(1)?.value();

	// 初始化线程池
	let mut pool_builder = ThreadPoolBuilder::new();
	match pool_builder.create() {
		Ok(val) => THREAD_POOL.set(val),
		Err(_) => final_rst = false,
	};

	// 初始化数据库连接池
	let init_block = async {
		match MySqlPool::connect(mysql_info).await {
			Some(val) => MYSQL_POOL.set(pool),
			None => final_rst = false,
		};
	};
	executor::block_on(init_block);

	// 初始化Redis连接池
	match redis::Client::open(redis_info) {
		Ok(val) => REDIS_POOL.set(val),
		Err(_) => final_rst = false
	};

	// 如果有回调函数，那么执行一下回调函数
	let args_length = cx.len();
	if args_length > 0 {
		let callback = cx.argument::<JsFunction>(0).unwrap();
		let null = cx.null();
		let args = vec![cx.number(0)];
		callback.call(&mut cx, null, args).unwrap();
	}
	// 返回结果
	Ok(cx.boolean(true))
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

	Ok(cx.string("hello mysql"))
}

register_module!(mut cx, {
	cx.export_function("init", init)?;
	cx.export_function("hello", hello)?;
	cx.export_function("calculate_max_win", calculate_max_win)?;
	Ok(())
});
