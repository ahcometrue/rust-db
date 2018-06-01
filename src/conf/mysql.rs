extern crate ini;
extern crate mysql;

use self::ini::Ini;
use self::mysql::{Opts,OptsBuilder};
use conf::config::*;
use conf::env::get_env;

pub struct MysqlConfig;

impl MysqlConfig{
pub fn new () -> Result<Opts, &'static str> {
        let mut builder = OptsBuilder::new();

        let _section_name = get_env() + ":common";
        let _conf = Ini::load_from_file(get_ini(Conf::Mysql)).unwrap();
        let _section = match _conf.section(Some(_section_name.to_owned())) {
            Some(arg) => arg,
            None => return Err("section value is null"),
        };
        match _section.get("a.host") {
            Some(arg) => builder.ip_or_hostname(Some(arg.to_string())),
            None => return Err("host value is null"),
        };
        match _section.get("a.port") {
            Some(arg) => builder.tcp_port(arg.parse::<u16>().unwrap()),
            None => return Err("port value is null"),
        };
        match _section.get("a.username") {
            Some(arg) => builder.user(Some(arg.to_string())),
            None => return Err("username value is null"),
        };
        match _section.get("a.password") {
            Some(arg) => builder.pass(Some(arg.to_string())),
            None => return Err("password value is null"),
        };
        match _section.get("a.database") {
            Some(arg) => builder.db_name(Some(arg.to_string())),
            None => return Err("database value is null"),
        };
        Ok(builder.into())
    }
}