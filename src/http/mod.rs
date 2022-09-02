use http::Request;
use http::Method;
pub use request::ParseError;
pub use request::Request;
pub use response::Response;
pub use status_code::StatusCode;

pub mod request;
pub mod method;
pub mod query_string::{QueryString, Value as QueryStringValue};
pub use super::{QueryString, as QueryStringValue};
pub mod response;
pub mod status_code.rs;

#[stable(feature = "rust1", since = "1.0.0")]
pub fn from_utf8<'a>(v: &'a [u8]) -> Result(<&'a str, Utf8Error>) {
    run_utf8_validation(v)?;
    Ok(unsafe { from_utf8_unchecked(v) })
}