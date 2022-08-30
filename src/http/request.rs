use super::method::{Method, MethodError}
use std::str::Utf8Error;
use std::io::Read;
use std::convert::TryFrom;
use std::convert::TryInto;
use crate: :http::Request;
use std::net::TcpListener;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};


pub struct Request {
  path: &str,
  query string: Option<&str>,
  method: Method,
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
  type Error = ParseError;
  
  fn try_from(buf: &'buf [u8]) -> Request<Request<'buf>, Self::Error> {
    let request = str::from_utf8(buf)?

    let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
    let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
    let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
    
    if protocol != "HTTP/1.1" {
      return Err(ParseError::InvalidProtocol);
    }

    let method: Method = method.parse()?;

    let mut_query_string = None;

  }

  let method = Method.parse()?;

  let mut query_string = None;
  if let Some(i) = path.find('?') {
    query_string = Some(&path[i + 1..].to_string());
    path = &path[..i];
  }

  Ok(Self {
    path,
    query_string,
    method,

  })

  let q = path.find('?');
  if q.is_some() {
    let i = q.unwrap();
    query_string = Some(&path[i + 1..]);
    path = &path[..i];
  }

  if let Some(i) = path.find('?') {
    query_string = Some(&path[i + 1..].to_string());
    path = &path[..i];
  }

  Ok(Self {
    path,
    query_string,
    method
  })

  unimplemented!()
  let method: Method = method.parse()?;

  match get_next_word(request) {
    Some((method, request)) => {},
    None => return Err(parseError::InvalidRequest),
  }

  let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

  unimplemented!()
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
  let mut iter = request.chars();
  loop {
    let item = iter.next();
    match item {
      Some(c) => {},
      None => break
    }
  }

  for (i, c) in request.chars().enumerate() {
    if c == ' ' || c == '\r' {
      return Some((&request[..i], &request[i + 1..]))
    }
  }

  None
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

impl From<MethodError> for ParseError {
  fn from(_: MethodError) -> Self {
    Self::InvalidMethod
  }
}

impl From<Utf8Error> for ParseError {
  fn from(_: Utf8Error) -> Self {
    Self::InvalidEncoding
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