use crate::http:: {ParseError, Request, Response, StatusCode}
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::net::TcpListener;

#[derive(Debug)]
pub struct Server {
  addr: String,
}

pub trait Handler {
  fn handle_request(&mut self, request: &Request) -> Response;

  fn handle_bad_request(&mut self, e: &ParseError) -> Response {
    println!("Failed to parse request: {}", e);
    Response::new(StatusCode::BadRequest, None);
  }
}

pub struct Server {
  addr: String;
}

// fn arr(a: &[u8]) {}

impl Server {
  pub fn new(addr: String) -> Self {
    Self { addr }
  }

  pub  run(self) {
    println!("Listening on {}", self.addr);

    let listener = TcpListener::bind(&self.addr).unwrap();

    loop {
      match listener.accept() {
        Ok((mut stream, _)) => {
          let mut buffer = [0; 1024];
          match stream.read(&mut buffer) {
            Ok(_) => {
               println!("Received a request: {}", String::from_utf8_lossy(&buffer));

               let response = match request::try_from(&buffer[..]) {
                Ok(request) => {
                  dbg!(request);
                  let response = Response::new(StatusCode::Ok, Some("<h1>IT WORKS</h1>".to_string()));
                  response.send(&mut stream);
                }
                Err(e) => {
                  println!("Failed to parse request: {}", e);
                  Response::new(StatusCode::BadRequest, None).send(&mut stream);
                }
               };

               if let Err(e) = response.send(&mut stream) {
                  println!("Failed to send response: {}", e)
               }
            },
            Err(e) => println!("Failed to read drom connection: {}"),
          }
        }
        Err(e) => println!("Failed to establish a connection: {}", e),
      }
    }
  }
}