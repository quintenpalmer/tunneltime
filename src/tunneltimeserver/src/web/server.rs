use hyper;
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
        let body = match req.uri().path() {
            "/health" => isetry!(handle_health()),
            "/api/towns" => isetry!(handle_town(&req, &conn)),
            _ => return path_not_found(req.uri().path()),
        };
        Box::new(futures::future::ok(
            Response::new()
                .with_header(ContentLength(body.len() as u64))
                .with_body(body),
        ))
    }
}

fn handle_health() -> Result<String, error::Error> {
    let res = serde_json::to_string(&serde_json::Value::Object(serde_json::Map::new()))?;
    Ok(res)
}

fn handle_town(_req: &Request, ds: &datastore::Datastore) -> Result<String, error::Error> {
    let serializable = ds.get_town(1)?;
    let res = serde_json::to_string(&serializable)?;
    Ok(res)
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
