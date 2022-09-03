#![allow(dead_code)];

use http::Method;
use http::Request;
use server::Server;
use website_handler::Websitehandler;

mod http;
mod server;
mod website_handler;

fn main() {
  let server = Server::new("127.00.1:8080".to_string());
  server.run(WebsiteHandler);
}

