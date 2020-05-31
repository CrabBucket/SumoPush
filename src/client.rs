

use std::net::{SocketAddr,UdpSocket};
use std::thread;
use super::Input;
use super::Packet;
pub fn any_array_size(array: Vec<u8>){

}

//Run client as all client needs to do is run the client portion
pub fn run_client(connection: SocketAddr){
    
    let temp = [0u8;32].to_vec();
    
    // Create socket to listen for server packets
    let localadr = "127.0.0.1:";
    let mut attempt = 3001;
    let mut append: String = String::from("3001");
    let mut socket;
    //The following loop basically just brute force looks for a valid socket by iterating over the port until we find a valid port starting from 3001
    loop {
        // bind returns our udp socket bind attempt   .join combines the localadr and our append attempt
        let mut res = UdpSocket::bind([localadr, &append].join(""));
        
        match res {
            Ok(sock) => {
                socket = sock;
                break;
            }
            Err(sock) => {
                attempt+=1;
                append = attempt.to_string();
            }
            
        }
    }

    
    
    

    
    // our data
    let bytes = "Now this is epic".as_bytes();
    
    //Test packet that is being sent to the server
    let testPacket = socket.send_to(bytes,"127.0.0.1:3001").expect("Error sending packets");
    println!("{:?}", testPacket);
    
    
    
    
    
}