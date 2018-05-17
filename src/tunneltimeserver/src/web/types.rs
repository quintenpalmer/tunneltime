use std::fmt;

use futures;
use hyper;
use hyper::header::ContentLength;
use hyper::server::Response;

use futures::future::Future;

pub type ResponseFuture = Box<Future<Item = Response, Error = hyper::Error>>;

const ISE: &'static str = "Internal Server Error";

pub fn five_hundred<T: fmt::Debug>(err: T) -> ResponseFuture {
    println!("{:?}", err);
    Box::new(futures::future::ok(
        Response::new()
            .with_status(hyper::StatusCode::InternalServerError)
            .with_header(ContentLength(ISE.len() as u64))
            .with_body(ISE),
    ))
}
