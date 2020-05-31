#![allow(non_snake_case)]
mod gamestate;
mod flags;
mod server;
mod client;

use flags::*;
use std::net::ToSocketAddrs;
const TIME_STEP: std::time::Duration = std::time::Duration::from_millis(10);

pub enum Input {
    Left = 1,
    Right = 2,
    Jump = 3,
    Dodge = 4,
    Charge = 5,
}



fn main() {

    let server = std::net::SocketAddr::from(([127,0,0,1],3001));
    let mut is_server = false;
    let mut port = 25565;
    if arg_count() > 2 {
        println!("Invalid number of arguments max number of arguments allowed is 2");
        std::process::exit(0);
    }
    for flag in get_flags() {
        match flag.flag_type {
            FlagType::Server => {
                is_server = true;
            }
            FlagType::Port => {
                port = flag.raw_value.parse::<i32>().unwrap();
            }
        }
    }
    if is_server {
        server::run_server(3001);
        std::process::exit(0);
    }
    client::run_client(server);


}
