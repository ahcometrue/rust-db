extern crate rust_db;
use rust_db::conf::env::{Conf, get_ini};
fn main() {
    let path = get_ini(Conf::Env);
    println!("{:?}", path);
}