use crate::http:: {ParseError, Request, Response, StatusCode}
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

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

impl Server {
  pub fn new(addr: String) -> Self {
    Self { addr }
  }

  pub run(self) {
    println!("Listening on {}", self.addr);

    let listener = TcpListener::bind(&self.addr).unwrap();

    loop {
      match listener.accept() {
        Ok((stream, _)) => {
          let a = 5;
          println!("OK");
        }
        Err(e) => println!("Failed to establish a connection: {}", e);
      }
    }
  }
}