extern crate rust_db;
use rust_db::conf::config::*;

fn main() {

let path = get_ini(Conf::Env);
println!("{:?}", path);



}