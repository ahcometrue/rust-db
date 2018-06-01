use std::path::PathBuf;
use conf::env;

#[derive(Debug)]
pub enum Conf {
   Mysql,
   Env,
   Redis,
   Mongo,
}


pub fn get_ini(conf: Conf) -> PathBuf {
    let mut path = env::get_conf_path();
    match conf {
        Conf::Env => path.push("env.ini"),
        Conf::Mysql => path.push("mysql_admin.ini"),
        Conf::Redis => path.push("redis.ini"),
        Conf::Mongo => path.push("mongo.ini"),
    };
    path
}
