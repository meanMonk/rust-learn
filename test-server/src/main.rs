use std::{io::Read, net::TcpListener};



pub fn main() {
  let listener = TcpListener::bind("127.0.0.1:8000").expect("cound not start server");
  
  for connection in listener.incoming() {
      match connection {
        Ok(mut stream) => {
            let mut text = String::new();
            stream.read_to_string(&mut text).expect("reading failed");
            println!("got from client {}", text);
            
        }
        Err(e) => {println!("connection failed {}",e)}
      }
  }
}