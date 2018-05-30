use std::env;
use std::path::{PathBuf};
use std::ffi::OsStr;


#[derive(Debug)]
pub enum Conf {
   Mysql,
   Env,
}

pub fn get_config(conf: Conf) -> PathBuf {
    let mut path = get_current_target_dir().parent().expect("target path").join("config/");
    match conf {
        Conf::Env => path.push("env.ini"),
        Conf::Mysql => path.push("mysql_admin.ini"),
    };
    path
}


fn get_current_target_dir() -> PathBuf {
    let bin = env::current_exe().expect("exe path");
    let mut target_dir = PathBuf::from(bin.parent().expect("bin parent"));
    while target_dir.file_name() != Some(OsStr::new("target")) {
        target_dir.pop();
    }
    target_dir
}
