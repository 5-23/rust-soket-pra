//서버 측 코드
use std::net::*;

//루프백 IP
const IP_ADDRESS:&str = "127.0.0.1:56789";

fn main()
{
    //리스너 생성
    let listener = TcpListener::bind(IP_ADDRESS).unwrap();

    println!("Start Server");

    for stream in listener.incoming() {
        if stream.is_ok() {
            println!("[log] connect");
        }
    }
}