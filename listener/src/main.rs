use std::io::prelude::*;
use std::net::{TcpStream,TcpListener};

//use tokio::io::{AsyncWriteExt,AsyncReadExt};
//use tokio::net::TcpStream;

const NEW_SERVER: &str = "127.0.0.1:7000";



 fn handle_connection (mut stream: TcpStream) {
        // read the buffer
        let mut buffer = [0;1024];
        let len = stream.read(&mut buffer).unwrap();
        let message = String::from_utf8_lossy(&mut buffer);
        println!("received {}", message);
        
        // write message
        let _ = stream.write_all(message.as_bytes());
        println!("connection established")
        
        
    }




#[warn(unused_mut)]
fn main() {
    // bind
    let listner =  TcpListener::bind(NEW_SERVER).unwrap();
    
    println!("listening in {:?}", listner);
    
    for stream in listner.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
        
    }
    
}
