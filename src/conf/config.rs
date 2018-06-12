extern crate ini;
extern crate mysql;

use self::ini::Ini;
use self::mysql::{Opts,OptsBuilder};
use std::fmt;
use std::result;
use conf::env::{Conf,get_env,get_ini};


#[derive(Debug)]
pub enum ConfigError {
    SECTION,
    HOST,
    PORT,
    USERNAME,
    PASSWORD,
    DATABASE,
}

type Result<T> = result::Result<T, ConfigError>;

pub struct MysqlConfig;

impl MysqlConfig{
    pub fn new () -> Result<Opts> {
        let mut builder = OptsBuilder::new();

        let _section_name = get_env() + ":common";
        let _conf = Ini::load_from_file(get_ini(Conf::Mysql)).unwrap();
        let _section = try!(_conf.section(Some(_section_name.to_owned())).ok_or(ConfigError::SECTION));

        let _host = try!(_section.get("a.host").ok_or(ConfigError::HOST));
        builder.ip_or_hostname(Some(_host.to_string()));

        let _port = try!(_section.get("a.port").ok_or(ConfigError::PORT));
        builder.tcp_port(_port.parse::<u16>().unwrap());

        let _username = try!(_section.get("a.username").ok_or(ConfigError::USERNAME));
        builder.user(Some(_username.to_string()));

        let _password = try!(_section.get("a.password").ok_or(ConfigError::PASSWORD));
        builder.pass(Some(_password.to_string()));

        let _database = try!(_section.get("a.database").ok_or(ConfigError::DATABASE));
        builder.db_name(Some(_database.to_string()));
        Ok(builder.into())
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigError::SECTION =>
                write!(f, "节点配置不存在"),
            ConfigError::HOST =>
                write!(f, "host为空"),
            ConfigError::PORT =>
                write!(f, "port为空"),
            ConfigError::USERNAME =>
                write!(f, "username为空"),
            ConfigError::PASSWORD =>
                write!(f, "password为空"),
            ConfigError::DATABASE =>
                write!(f, "database为空"),
        }
    }
}
