// SERVER
use std::{net::*, io::{Read, Write}, str::from_utf8, thread};

const IP_ADDRESS:&str = "127.0.0.1:56789";

fn handle(mut stream: TcpStream)
{
    let mut input = [0 as u8; 100];

    while match stream.read(&mut input) {
        Ok(_) => {
            println!("{}: {}", stream.peer_addr().unwrap(), from_utf8(&input).unwrap());
            stream.write_all(&input).unwrap();
            stream.flush().unwrap();
            true
        }
        Err(e) => {
            println!("DISCONNECTED {e}");
            false
        }
    } {}

}

fn main()
{
    let listener = TcpListener::bind(IP_ADDRESS).unwrap();

    println!("Start Server");

    for stream in listener.incoming() {
        if stream.is_ok() {
            thread::spawn(|| handle(stream.unwrap()));
        }
    }
}