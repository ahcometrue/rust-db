extern crate rust_db;
use rust_db::conf::config::MysqlConfig;
use std::process;


extern crate mysql;
use mysql as my;

fn main() {
    let opts = MysqlConfig::new().unwrap_or_else(|err| {
        println!("ini文件有误: {}", err);
        process::exit(1);
    });
    let pool = my::Pool::new(opts).unwrap_or_else(|err| {
        println!("mysql连接失败: {}", err);
        process::exit(1);
    });
    println!("{:?}", pool);
}