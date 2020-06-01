// Gamestate needed in server because server updates the gamestate and will likely send it to clients
use super::gamestate::{Sumo,Game,Floor,init_game};
//Udp Sockets required because server has to listen on socket
use std::net::{UdpSocket,SocketAddr};
// Maybe multithreaded server in the future not sure
use std::thread;
//Temporary import for testing
use std::str;

//Run server is essentially the entire server I might split things apart later but for now we just want to run the server
pub fn run_server(port: i32) {
    //This is initiates the gamestate from gamestate.rs
    let mut game = init_game();
    // create the socket
    //This is the uncertain socket (maybe the socket is already being listened on) 
    //we temporarily are listening on in the future I think I might need to find the local ipv4
    let res = UdpSocket::bind("127.0.0.1:3001");
    //Instantiate socket that we will use later
    let socket;
    //We match the result of the socket bind attempt to either pass or fail and we either accept the socket or exit the process.
    match res {
        Ok(sock) => {
            socket = sock;
        }
        Err(sock) => {
            println!("Could not bind socket at: {} exiting",sock);
            std::process::exit(0);
        }
    }
    //Temp sumos for testing
    let mut left = Sumo::new(12,12,20,20);
    let mut right = Sumo::new(12,12,20,20);
    //Buffers are going to be used to recieve packets from the clients.
    //In the future we will need both packets from the client at the time step to update the server gamestate
    let leftBuffer: Vec<[u8; 64]> = Vec::new();
    let rightBuffer: Vec<[u8; 64]> = Vec::new();
    //Main recievie loop that listens for packets on the socket
    loop {
        //The data we are going to recieve from the packet
        let data: &mut [u8] = &mut vec![0u8; 64];
        //We either get a valid packet or we don't
        match socket.recv(data) {
            //For now I just print the packet as utf string but this will be a client packet in the futre
            Ok(_) => {
                println!("{}",str::from_utf8(data).unwrap());
            }
            //Log the error
            Err(_) =>{
                println!("Error on recieve");
            }
        }
    }
}