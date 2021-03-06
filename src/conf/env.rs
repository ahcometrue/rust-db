extern crate ini;

use self::ini::Ini;
use std::ffi::OsStr;
use std::env;
use std::path::PathBuf;

const CONFIG_DIR : &'static str = "config/";

pub fn get_env() -> String {
    let conf = Ini::load_from_file(get_ini(Conf::Env)).unwrap();
    let section = conf.section(Some("env".to_owned())).unwrap();
    let env = section.get("env").unwrap();
    env.to_string()
}

fn get_conf_path() -> PathBuf {
    get_current_target_dir().parent().expect("target path").join(CONFIG_DIR)
}


fn get_current_target_dir() -> PathBuf {
    let bin = env::current_exe().expect("exe path");
    let mut target_dir = PathBuf::from(bin.parent().expect("bin parent"));
    while target_dir.file_name() != Some(OsStr::new("target")) {
        target_dir.pop();
    }
    target_dir
}


#[derive(Debug)]
pub enum Conf {
   Mysql,
   Env,
   Redis,
   Mongo,
}

pub fn get_ini(conf: Conf) -> PathBuf {
    let mut path = get_conf_path();
    match conf {
        Conf::Env => path.push("env.ini"),
        Conf::Mysql => path.push("mysql_admin.ini"),
        Conf::Redis => path.push("redis.ini"),
        Conf::Mongo => path.push("mongo.ini"),
    };
    path
}