

use std::net::{SocketAddr,UdpSocket};
use std::thread;
use super::Input;


#[derive(Debug,PartialEq,Clone)]
pub struct Packet {
    inputs: [u8;3],
    packetnumber: u32,
    inputbuffer: [[u8;3];5],
}
impl Packet {
    pub fn test() {
        let testpacket = Packet{
            inputs: [1u8;3],
            packetnumber: 33,
            inputbuffer: [[5u8;3];5]
        };
        assert!(testpacket == Packet::deserialize(&(testpacket.clone().serialize())));
    }
    pub fn serialize(self) -> Vec<u8> {
        [self.inputs.iter()
                    .copied()
                    .collect(),
        self.packetnumber.to_le_bytes().to_vec(),
        self.inputbuffer.iter()
                        .flatten()
                        .copied()
                        .collect()
        ].concat()
    }
    pub fn deserialize(data: &[u8]) -> Packet {
        let inputs = [data[0],data[1],data[2]];

        let packetnumbervec = &data[3..7];
        let packetnumber = u32::from_le_bytes([packetnumbervec[0],packetnumbervec[1],packetnumbervec[2],packetnumbervec[3]]);
        let mut inputbuffer = [[0u8;3];5];
        let mut index = 7;
        for x in 0..=4 {
            for y in 0..=2{
                inputbuffer[x][y] = data[index];
                index+=1;
            }
        }

        Packet{
            inputs: inputs,
            packetnumber: packetnumber,
            inputbuffer: inputbuffer
        }
    }
}

//Run client as all client needs to do is run the client portion
pub fn run_client(connection: SocketAddr){
    
    let temp = [0u8;32].to_vec();
    
    Packet::test();

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