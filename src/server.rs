
use super::gamestate::{Sumo,Game,Floor,init_game};
use std::net::{UdpSocket,SocketAddr};
use std::thread;
use std::str;


pub fn run_server(port: i32) {
    let mut game = init_game();
    // create the socket
    let res = UdpSocket::bind("127.0.0.1:3001");
    let socket;
    match res {
        Ok(sock) => {
            socket = sock;
        }
        Err(sock) => {
            println!("Could not bind socket at: {} exiting",sock);
            std::process::exit(0);
        }
    }
    let mut left = Sumo::new(12f32,12f32,20,20);
    let mut right = Sumo::new(12f32,12f32,20,20);
    let mut floor = Floor{}; 
    loop {
        let data: &mut [u8] = &mut vec![0u8; 64];
        match socket.recv(data) {
            Ok(_) => {
                println!("{}",str::from_utf8(data).unwrap());
            }
            Err(_) =>{
                println!("Error on recieve");
            }
        }
    }
}