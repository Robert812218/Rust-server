use super::server::Handler;
use super::http::{Request, Response, StatusCode};
pub struct Websitehandler;


impl Handler for Websitehandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        Response::new(StatusCode::Ok, Some("<h1>TEST</h1>".to_string()));
        server.run();
    }
}