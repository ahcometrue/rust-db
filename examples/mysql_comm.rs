extern crate rust_db;
use rust_db::config::mysql::Mysql as MysqlConfig;
use std::process;


extern crate mysql;
use mysql as my;

fn main() {
    let config = MysqlConfig::new().unwrap_or_else(|err| {
        println!("ini文件有误: {}", err);
        process::exit(1);
    });
    let comm = config.driver + "://" + &config.username + ":" +
    	&config.password + "@" + &config.host + ":" + &config.port.to_string();
    let pool = my::Pool::new(comm).unwrap_or_else(|err| {
        println!("mysql连接失败: {}", err);
        process::exit(1);
    });
    println!("{:?}", pool);


}