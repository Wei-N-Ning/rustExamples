use std::net::UdpSocket;
use std::env;

const MAX_UDP_PAYLOAD_SIZE : usize = 65507;

fn start_client(msg : String) -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:23456")?;
    let mut buffer = [0; MAX_UDP_PAYLOAD_SIZE+1];

    socket.connect("10.0.50.110:9999")?;

    println!("[CLIENT] Local address: {:?}", socket.local_addr());
    match socket.send(msg.as_bytes()) {
        Ok(bytes_sent) => {
            println!("[CLIENT] {} bytes sent.", bytes_sent);
         },
        Err(e) => println!("[CLIENT] Error writing to socket: {}", e),
    }

    let (bytes_read, peer_addr) = socket.recv_from(&mut buffer)?;
    match String::from_utf8(buffer.to_vec()) {
        Ok(msg) => {
            println!("[CLIENT]: {}", msg);
            println!("[CLIENT] {} bytes read.", bytes_read);
        }
        Err(e) => { 
            println!("[CLIENT]: Error decoding message {}", e);
        }
    }

    Ok(())
}

fn main() {
    match start_client("S|".to_string()) {
        Ok(_) => { },
        Err(e) => { 
            println!("[CLIENT] Error: {}", e);
        }
    }
}