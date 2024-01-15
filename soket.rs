//클라이언트 측 코드
use std::net::*;

const IP_ADDRESS: &str = "127.0.0.1:56789";

fn main()
{
    //그냥 연결만 함
    let stream =
        TcpStream::connect(IP_ADDRESS)unwrap();
}