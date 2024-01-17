// CLIENT1
use std::{net::*, io::{Write, Read}, thread, sync::Mutex, str::from_utf8};

const IP_ADDRESS: &str = "127.0.0.1:56789";

fn main()
{
    let mut stream =
        TcpStream::connect(IP_ADDRESS).unwrap();

    loop {
        let mut input = [0 as u8; 100];
        stream.read(&mut input).unwrap();
        println!("[???] {:?}", from_utf8(&input));
    }

}