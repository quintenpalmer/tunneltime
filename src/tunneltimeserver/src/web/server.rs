use hyper;
use serde;
use serde_json;

use futures;

use hyper::header::ContentLength;
use hyper::server::{Request, Response, Service};

use error;
use web::types;

use datastore;

pub struct Handler;

impl Service for Handler {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = types::ResponseFuture;

    fn call(&self, req: Request) -> Self::Future {
        let conn = isetry!(datastore::Datastore::new(
            "localhost".to_string(),
            "tunneltime",
            5432,
            "tunneltime_user",
            None,
        ));
        let resp = match req.uri().path() {
            "/health" => isetry!(handle_health()),
            "/api/towns" => isetry!(handle_town(&req, &conn)),
            "/api/dwarves" => isetry!(handle_dwarves(&req, &conn)),
            _ => return path_not_found(req.uri().path()),
        };
        resp
    }
}

fn handle_health() -> Result<types::ResponseFuture, error::Error> {
    build_response(&serde_json::Value::Object(serde_json::Map::new()))
}

fn handle_town(
    _req: &Request,
    ds: &datastore::Datastore,
) -> Result<types::ResponseFuture, error::Error> {
    build_response(ds.get_town(1)?)
}

fn handle_dwarves(
    _req: &Request,
    ds: &datastore::Datastore,
) -> Result<types::ResponseFuture, error::Error> {
    build_response(ds.get_dwarves(1)?)
}

fn build_response<T: serde::Serialize>(ser: T) -> Result<types::ResponseFuture, error::Error> {
    let body = serde_json::to_string(&ser)?;
    Ok(Box::new(futures::future::ok(
        Response::new()
            .with_header(ContentLength(body.len() as u64))
            .with_body(body),
    )))
}

const ROUTE_NOT_FOUND: &'static str = "Route Not Found";

fn path_not_found(path: &str) -> types::ResponseFuture {
    println!("{:?}", path);
    Box::new(futures::future::ok(
        Response::new()
            .with_status(hyper::StatusCode::NotFound)
            .with_header(ContentLength(ROUTE_NOT_FOUND.len() as u64))
            .with_body(ROUTE_NOT_FOUND),
    ))
}
