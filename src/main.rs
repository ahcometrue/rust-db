extern crate ini;

use ini::Ini;
use std::process;

#[derive(Debug)]
pub struct MysqlConfig {
    driver: String,
    host: String,
    port: u16,
    username: String,
    password: String,
    database: String,

}

impl MysqlConfig {
    fn new () -> Result<MysqlConfig, &'static str> {
        let section_name = get_evn() + ":common";
        let conf = Ini::load_from_file("../conf/mysql_admin.ini").unwrap();
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
        Ok(MysqlConfig {
            driver,
            host,
            port,
            username,
            password,
            database,
        })
    }
}


fn main() {
    let config = MysqlConfig::new().unwrap_or_else(|err| {
        println!("ini文件有误: {}", err);
        process::exit(1);
    });
    print!("{:?}", config.host);

}

fn get_evn() -> String {
    let conf = Ini::load_from_file("../conf/evn.ini").unwrap();
    let section = conf.section(Some("evn".to_owned())).unwrap();
    let evn = section.get("evn").unwrap();
    evn.to_string()
}
