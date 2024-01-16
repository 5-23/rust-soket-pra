// CLIENT
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
    let stream =
        Mutex::new(TcpStream::connect(IP_ADDRESS).unwrap());

    thread::scope(|scoop| {
        scoop.spawn(|| {
            println!("Start 1st therad");
            loop {
                let checker = stream.lock();
                match checker {
                    Ok(x) => {
                        let mut stream_witer = x;
                        let mut msg = String::new();
                        std::io::stdin().read_line(&mut msg).unwrap();
                        stream_witer.write(&msg.as_bytes()).unwrap();
                    }
                    Err(_) => {}
                }
            }
        });
        scoop.spawn(|| {
            println!("Start 2nd therad");
            loop {
                let checker = stream.lock();
                match checker {
                    Ok(x) => {
                        let mut stream_writer = x;
                        let mut input = String::new();
                        stream_writer.read_to_string(&mut input).unwrap();
                        println!("[???] {input}");
                    }
                    Err(_) => {
                        println!("j")
                    }
                    
                }
            }
        });
    });
}