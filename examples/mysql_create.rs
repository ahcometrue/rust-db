extern crate rust_db;
#[macro_use]
extern crate mysql;

use rust_db::conf::config::MysqlConfig;
use std::process;
use mysql as my;
use std::sync::{Mutex, Arc, MutexGuard};
use std::thread;
use std::ops::Deref;

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}


fn main() {
    let opts = MysqlConfig::new().unwrap_or_else(|err| {
        println!("ini文件有误: {}", err);
        process::exit(1);
    });
    let pool = my::Pool::new(opts).unwrap_or_else(|err| {
        println!("mysql连接失败: {}", err);
        process::exit(1);
    });
    pool.prep_exec(r"CREATE TABLE payment (
                         id int unsigned NOT NULL AUTO_INCREMENT,
                         customer_id int not null,
                         amount int not null,
                         account_name text,
                         PRIMARY KEY (id)
                     )", ()).unwrap();
    let mysql_pool = Arc::new(Mutex::new(pool));
    let mut handles = vec![];
    for _ in 0..10 {
        let mysql_pool = Arc::clone(&mysql_pool);
        let handle = thread::spawn(move || {
            let mut _mysql_pool = mysql_pool.lock().unwrap();
            handle_connection(_mysql_pool);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

fn handle_connection(pool: MutexGuard<my::Pool>) {
    let pool = pool.deref();
    println!("{:?}", pool);
    let payments = vec![
        Payment { customer_id: 1, amount: 2, account_name: None },
        Payment { customer_id: 3, amount: 4, account_name: Some("foo".into()) },
        Payment { customer_id: 5, amount: 6, account_name: None },
        Payment { customer_id: 7, amount: 8, account_name: None },
        Payment { customer_id: 9, amount: 10, account_name: Some("bar".into()) },
    ];
    for mut stmt in pool.prepare(r"INSERT INTO payment
                                       (customer_id, amount, account_name)
                                   VALUES
                                       (:customer_id, :amount, :account_name)").into_iter() {
        for p in payments.iter() {
            stmt.execute(params!{
                "customer_id" => p.customer_id,
                "amount" => p.amount,
                "account_name" => &p.account_name,
            }).unwrap();
        }
    }
}