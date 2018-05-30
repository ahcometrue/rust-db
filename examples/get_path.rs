extern crate rust_db;
use rust_db::config::config::*;

fn main() {

let path = get_config(Conf::Env);
println!("{:?}", path);



}