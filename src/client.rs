

use std::net::{SocketAddr,UdpSocket};
use std::thread;
use super::Input;





pub fn run_client(connection: SocketAddr){
    
    
    
    // Create socket to listen for server packets
    let localadr = "127.0.0.1:";
    let mut attempt = 3001;
    let mut append = "3001";
    let mut socket;
    loop {
        let mut res = UdpSocket::bind([localadr, append].join(""));
        
        match res {
            Ok(sock) => {
                socket = sock;
                break;
            }
            Err(sock) => {
                attempt+=1;
                append = &attempt.to_string();
            }
            
        }
    }

    
    
    

    
    // our data
    let bytes = "Now this is epic".as_bytes();
    
    // You can create packets with different reliabilities
    let testPacket = socket.send_to(bytes,"127.0.0.1:3001").expect("Error sending packets");
    println!("{:?}", testPacket);
    
    
    
    
    
}