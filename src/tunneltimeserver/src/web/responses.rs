use futures;
use hyper;

use hyper::header::ContentLength;
use hyper::server::Response;
use web::types;

const RESOURCE_NOT_FOUND: &'static str = r#"{"message":"Resource Not Found"}"#;

pub fn resource_not_found() -> types::ResponseFuture {
    println!("resource not found");
    Box::new(futures::future::ok(
        Response::new()
            .with_status(hyper::StatusCode::NotFound)
            .with_header(ContentLength(RESOURCE_NOT_FOUND.len() as u64))
            .with_body(RESOURCE_NOT_FOUND),
    ))
}

const ROUTE_NOT_FOUND: &'static str = r#"{"message":"Route Not Found"}"#;

pub fn path_not_found(path: &str) -> types::ResponseFuture {
    println!("{:?}", path);
    Box::new(futures::future::ok(
        Response::new()
            .with_status(hyper::StatusCode::NotFound)
            .with_header(ContentLength(ROUTE_NOT_FOUND.len() as u64))
            .with_body(ROUTE_NOT_FOUND),
    ))
}

const METHOD_NOT_ALLOWED: &'static str = r#"{"message":"Method Not Allowed"}"#;

pub fn method_not_allowed(method: &hyper::Method) -> types::ResponseFuture {
    println!("{}", method);
    Box::new(futures::future::ok(
        Response::new()
            .with_status(hyper::StatusCode::MethodNotAllowed)
            .with_header(ContentLength(METHOD_NOT_ALLOWED.len() as u64))
            .with_body(METHOD_NOT_ALLOWED),
    ))
}

const BAD_REQUEST: &'static str = r#"{"message":"Bad Request"}"#;

pub fn bad_request(message: &str) -> types::ResponseFuture {
    println!("{}", message);
    Box::new(futures::future::ok(
        Response::new()
            .with_status(hyper::StatusCode::BadRequest)
            .with_header(ContentLength(BAD_REQUEST.len() as u64))
            .with_body(BAD_REQUEST),
    ))
}
