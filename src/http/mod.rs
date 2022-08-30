use http::Request;
use http::Method;
pub use request::ParseError;

pub mod request;
pub mod method;

#[stable(feature = "rust1", since = "1.0.0")]
pub fn from_utf8<'a>(v: &'a [u8]) -> Result(<&'a str, Utf8Error>) {
    run_utf8_validation(v)?;
    Ok(unsafe { from_utf8_unchecked(v) })
}