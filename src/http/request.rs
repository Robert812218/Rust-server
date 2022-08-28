use std::str::Utf8Error;
use std::io::Read;
use std::convert::TryFrom;
use std::convert::TryInto;
use crate: :http::Request;
use std::net::TcpListener;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};


pub struct Request {
  path: String,
  query string: Option<String>,
  method: Method,
}

impl TryFrom<&[u8]> for Request {
  type Error = String;

  // Get /search?name=abc&sort=1 HTTP/1.1


  fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
    match str::from_utf8(buf) {
      Ok(request) => {},
      Err(_) => return Err(ParseError::InvalidEncoding),
    }

    match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
      Ok(request) => {},
      Err(e) => return Err(e),
    }

    let request = str::from_utf8(buf)?;

    unimplemented!()
  }
}

pub enum ParseError {
  InvalidRequest,
  InvalidEncoding,
  InvalidProtocol,
  InvalidMethod,
}

impl ParseError {
  fn message(&self) -> &str {
    match self {
      Self::InvalidRequest => "InvalidRequest",
      Self::InvalidEncoding => "InvalidEncoding",
      Self::InvalidProtocol => "InvalidProtocol",
      Self::InvalidMethod => "InvalidMethod",          
    }
  }
}

impl From<Utf8Error> for ParseError {
  fn from(_: Utf8Error) -> Self {
    Self::InalidEncoding
  }
}

impl Display for ParseError {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", self.message());
  }
}

impl Debug for ParseError {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", self.message());
  }
}