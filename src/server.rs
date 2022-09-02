use crate::http:: {ParseError, Request, Response, StatusCode}
use std::convert::TryFrom;
use std::io::Read;
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
            },
            Err(e) => println!("Failed to read drom connection: {}");
          }
        }
        Err(e) => println!("Failed to establish a connection: {}", e);
      }
    }
  }
}