use super::method::Method;

struct Request {
  path: String,
  query_string: Option<String>,
  method: super::method::Method,
}