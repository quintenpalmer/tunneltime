use std::fmt;

use hyper;
use serde_json;

use futures;
use futures::future::Future;

use hyper::header::ContentLength;
use hyper::server::{Request, Response, Service};

use tunneltimecore::models;

pub struct Handler;

impl Service for Handler {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, _req: Request) -> Self::Future {
        let mut town = models::Town::new();
        town.acquire_gem_shop();
        let body = match serde_json::to_string(&town) {
            Ok(val) => val,
            Err(err) => return five_hundred(err),
        };
        println!("log: sending town");
        Box::new(futures::future::ok(
            Response::new()
                .with_header(ContentLength(body.len() as u64))
                .with_body(body),
        ))
    }
}

const ISE: &'static str = "Internal Server Error";

fn five_hundred<T: fmt::Debug>(err: T) -> Box<Future<Item = Response, Error = hyper::Error>> {
    println!("{:?}", err);
    Box::new(futures::future::ok(
        Response::new()
            .with_header(ContentLength(ISE.len() as u64))
            .with_body(ISE),
    ))
}
