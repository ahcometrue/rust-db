use std::env;
use std::path::{PathBuf};


#[derive(Debug)]
pub enum Conf {
   Mysql,
   Env,
}

pub fn get_config(conf: Conf) -> PathBuf {
    let mut path = env::current_exe().unwrap()
    	.parent().unwrap()
    	.join("../../config/");

    match conf {
        Conf::Env => path.push("env.ini"),
        Conf::Mysql => path.push("mysql_admin.ini"),
    };
    path
}


