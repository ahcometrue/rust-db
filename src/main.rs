extern crate rust_db;
use rust_db::config::mysqlconfig::MysqlConfig;

use std::process;


fn main() {
    let config = MysqlConfig::new().unwrap_or_else(|err| {
        println!("ini文件有误: {}", err);
        process::exit(1);
    });
    print!("{:?}", config.host);

}


