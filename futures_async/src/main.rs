#[warn(non_snake_case)]
fn test() {
    const NUM_MESSAGES: usize = 1000;
    let receiver = async {
        async_std::net::UdpSocket::bind("127.0.0.1:0")
        .await
        .unwrap()};
        
        
    let sender = async {
        async_std::net::UdpSocket::bind("127.0.0.1:0")
        .await
        .unwrap()};
    
    
    
    let sender_future = async {
        for _ in 0..NUM_MESSAGES {
    let _ = sender.send_to(b"hello world", receiver.local_addr().unwrap()).await;
    futures_timer::Delay::new(std::time::Duration::from_millis(1)).await;
    //std::thread::sleep(std::time::Duration::from_millis(1));
    
    }
    
    };
    
    
    let receiver_future = async {
         let mut buffer = [0;256];
    let mut count = 0;
    for _ in 0..NUM_MESSAGES {
    let _ = receiver.recv_from(&mut buffer).unwrap();
    count += 1;
    println!("We received the message {}", count);
    }
    };
    
    futures::executor::block_on(async{
        futures::join!(sender_future,receiver_future);
        // let _ = sender_future.await;
        // let _ = receiver_future.await;
    })
   
    
}


fn main() {
    test();
    println!("Hello, world!");
}
