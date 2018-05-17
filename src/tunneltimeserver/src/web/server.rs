use std::fmt;

use hyper;
use serde_json;

use futures;
use futures::future::Future;

use hyper::header::ContentLength;
use hyper::server::{Request, Response, Service};

use error;

use datastore;

pub struct Handler;

type ResponseFuture = Box<Future<Item = Response, Error = hyper::Error>>;

impl Service for Handler {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = ResponseFuture;

    fn call(&self, req: Request) -> Self::Future {
        let conn = match datastore::Datastore::new(
            "localhost".to_string(),
            "tunneltime",
            5432,
            "tunneltime_user",
            None,
        ) {
            Ok(val) => val,
            Err(err) => return five_hundred(err),
        };
        let body = match req.uri().path() {
            "/api/towns" => match handle_town(&req, &conn) {
                Ok(val) => val,
                Err(err) => return five_hundred(err),
            },
            _ => return path_not_found(req.uri().path()),
        };
        Box::new(futures::future::ok(
            Response::new()
                .with_header(ContentLength(body.len() as u64))
                .with_body(body),
        ))
    }
}

fn handle_town(_req: &Request, ds: &datastore::Datastore) -> Result<String, error::Error> {
    let serializable = ds.get_town(1)?;
    let res = serde_json::to_string(&serializable)?;
    Ok(res)
}

const ISE: &'static str = "Internal Server Error";
const ROUTE_NOT_FOUND: &'static str = "Route Not Found";

fn five_hundred<T: fmt::Debug>(err: T) -> ResponseFuture {
    println!("{:?}", err);
    Box::new(futures::future::ok(
        Response::new()
            .with_status(hyper::StatusCode::InternalServerError)
            .with_header(ContentLength(ISE.len() as u64))
            .with_body(ISE),
    ))
}

fn path_not_found(path: &str) -> ResponseFuture {
    println!("{:?}", path);
    Box::new(futures::future::ok(
        Response::new()
            .with_status(hyper::StatusCode::NotFound)
            .with_header(ContentLength(ROUTE_NOT_FOUND.len() as u64))
            .with_body(ROUTE_NOT_FOUND),
    ))
}
