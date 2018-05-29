extern crate ini;

use self::ini::Ini;
use config::config::{Conf, get_config};

pub fn get_env() -> String {
    let conf = Ini::load_from_file(get_config(Conf::Env)).unwrap();
    let section = conf.section(Some("env".to_owned())).unwrap();
    let env = section.get("env").unwrap();
    env.to_string()
}