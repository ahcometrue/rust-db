extern crate ini;
use self::ini::Ini;

pub fn get_evn() -> String {
    let conf = Ini::load_from_file("../conf/evn.ini").unwrap();
    let section = conf.section(Some("evn".to_owned())).unwrap();
    let evn = section.get("evn").unwrap();
    evn.to_string()
}