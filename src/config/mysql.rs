extern crate ini;

use self::ini::Ini;
use config::config::{Conf, get_config};
use config::env::get_env;

#[derive(Debug)]
pub struct Mysql {
    pub driver: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,

}

impl Mysql {
    pub fn new () -> Result<Mysql, &'static str> {
        let section_name = get_env() + ":common";
        let conf = Ini::load_from_file(get_config(Conf::Mysql)).unwrap();
        let section = match conf.section(Some(section_name.to_owned())) {
            Some(arg) => arg,
            None => return Err("section value is null"),
        };
        let driver = match section.get("a.driver") {
            Some(arg) => arg.to_string(),
            None => return Err("driver value is null"),
        };
        let host = match section.get("a.host") {
            Some(arg) => arg.to_string(),
            None => return Err("host value is null"),
        };
        let port = match section.get("a.port") {
            Some(arg) => arg.parse::<u16>().unwrap(),
            None => return Err("port value is null"),
        };
        let username = match section.get("a.username") {
            Some(arg) => arg.to_string(),
            None => return Err("username value is null"),
        };
        let password = match section.get("a.password") {
            Some(arg) => arg.to_string(),
            None => return Err("password value is null"),
        };
        let database = match section.get("a.database") {
            Some(arg) => arg.to_string(),
            None => return Err("database value is null"),
        };
        Ok(Mysql {
            driver,
            host,
            port,
            username,
            password,
            database,
        })
    }
}