use std::io::prelude::*;
use std::net::TcpListener;
use tokio::io::{AsyncWriteExt,AsyncReadExt};
use tokio::net::TcpStream;



const ECHO_SERVER_ADDRESS: &str = "localhost:1234";

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
#[tokio::main]
async fn main() {
    // println!("Connecting to {}", ECHO_SERVER_ADDRESS);
    
    
    // bind
    let listner =  TcpListener::bind(NEW_SERVER).unwrap();
    
    println!("listening in {:?}", listner);
    
    for stream in listner.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
        
    }
    
   
    
    
    
    
    
    
    
    
        // Connected
    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRESS).await {
        println!("conneted to echoserver {}:{}", stream.local_addr().unwrap().ip(), stream.local_addr().unwrap().port());
        
        // write a message to echo server
        let message = "async version hello world and hello tcp";
        let _ = stream.write_all(message.as_bytes()).await;
        //let _ = stream.flush();// write_all can be used instead of flush
        println!("sent {}",message);
        
        
        // read the result
        let mut buffer = [0;1024];
        let data_len_inbytes = stream.read(&mut buffer).await.unwrap();
        let read_message = String::from_utf8_lossy(&mut buffer);
        println!("Data back from echo server {} and message is {}", data_len_inbytes, read_message);
    
    
    
    } else {
        println!("failed to connect")
    }
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    //                             Standard non-tokio version 
    // // Connected
    // if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRESS) {
    //     println!("conneted to echoserver {}:{}", stream.local_addr().unwrap().ip(), stream.local_addr().unwrap().port());
        
    //     // write a message to echo server
    //     let message = "hello world and hello tcp";
    //     let _ = stream.write(message.as_bytes());
    //     let _ = stream.flush();
    //     println!("sent {}",message);
        
        
    //     // read the result
    //     let mut buffer = [0;1024];
    //     let data_len_inbytes = stream.read(&mut buffer).unwrap();
    //     let read_message = String::from_utf8_lossy(&mut buffer);
    //     println!("Data back from echo server {} and message is {}", data_len_inbytes, read_message);
    
    
    
    // } else {
    //     println!("failed to connect")
    // }
    
    
   


}
