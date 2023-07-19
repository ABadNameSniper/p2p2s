use std::{
    fs,
    net::TcpStream, io::Write, io::Read
};


fn main() -> () {
    let server_address = fs::read_to_string("./server_address.txt").unwrap();

    let mut stream = TcpStream::connect(server_address).unwrap();

    stream.write(&[1]).unwrap();
    stream.read(&mut [0; 128]).unwrap();
    
}
