use std::io;

use hyper;
use serde;
use serde_json;

use futures;
use futures::{Future, Stream};

use hyper::header::ContentLength;
use hyper::server::{Request, Response, Service};

use tunneltimecore::models;

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
            "/health" => handle_health(),
            "/api/users" => match req.method() {
                hyper::Method::Get => handle_user_get(req, &conn),
                hyper::Method::Post => handle_user_post(req, conn),
                _ => return method_not_allowed(req.method()),
            },
            "/api/towns" => handle_town(&req, &conn),
            "/api/dwarves" => handle_dwarves(&req, &conn),
            _ => return path_not_found(req.uri().path()),
        };
        resp
    }
}

fn handle_health() -> types::ResponseFuture {
    build_response(&serde_json::Value::Object(serde_json::Map::new()))
}

fn handle_user_get(_req: Request, ds: &datastore::Datastore) -> types::ResponseFuture {
    let user = isetry!(ds.get_user("postprompt".to_string()));
    build_response(user)
}

fn handle_user_post(req: Request, ds: datastore::Datastore) -> types::ResponseFuture {
    Box::new(req.body().concat2().and_then(move |chunk| {
        let v: models::NewUser = isetry!(
            serde_json::from_slice(&chunk).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        );
        let user = isetry!(ds.new_user(v.user_name));
        build_response(user)
    }))
}

fn handle_town(_req: &Request, ds: &datastore::Datastore) -> types::ResponseFuture {
    build_response(isetry!(ds.get_town(1)))
}

fn handle_dwarves(_req: &Request, ds: &datastore::Datastore) -> types::ResponseFuture {
    build_response(isetry!(ds.get_dwarves(1)))
}

fn build_response<T: serde::Serialize>(ser: T) -> types::ResponseFuture {
    let body = isetry!(serde_json::to_string(&ser));
    Box::new(futures::future::ok(
        Response::new()
            .with_header(ContentLength(body.len() as u64))
            .with_body(body),
    ))
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

const METHOD_NOT_ALLOWED: &'static str = "Method Not Allowed";

fn method_not_allowed(method: &hyper::Method) -> types::ResponseFuture {
    println!("{}", method);
    Box::new(futures::future::ok(
        Response::new()
            .with_status(hyper::StatusCode::MethodNotAllowed)
            .with_header(ContentLength(METHOD_NOT_ALLOWED.len() as u64))
            .with_body(METHOD_NOT_ALLOWED),
    ))
}
