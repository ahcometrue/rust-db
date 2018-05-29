extern crate rust_db;
use rust_db::login::user::login;
use std::process;


fn main() {
    let login = login("hg", "pyyx123").unwrap_or_else(|err| {
        println!("登陆失败: {}", err);
        process::exit(1);
    });;
    if login {
    	println!("登陆成功！");
    }
}
