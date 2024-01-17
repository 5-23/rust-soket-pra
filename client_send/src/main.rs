// CLIENT2
use std::{net::*, io::{Write, Read}, thread, sync::Mutex, str::from_utf8};

const IP_ADDRESS: &str = "127.0.0.1:56789";


// fn handle(mut stream: TcpStream)
// {
//     let mut input = [0 as u8; 100];

//     while match stream.read(&mut input) {
//         Ok(_) => {
//             println!("{}: {}", stream.peer_addr().unwrap(), from_utf8(&input).unwrap());
//             true
//         }
//         Err(e) => {
//             println!("DISCONNECTED {e}");
//             false
//         }
//     } {}

// }


fn main()
{

    let mut stream =
        TcpStream::connect(IP_ADDRESS).unwrap();
    loop {
        let mut msg = String::new();
        std::io::stdin().read_line(&mut msg).unwrap();
        let a = stream.write(&msg.as_bytes()).unwrap();
        println!("{a}")
    }
        
}