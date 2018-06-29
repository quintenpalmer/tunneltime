use std::collections::HashMap;
use std::io;
use std::str::FromStr;

use hyper;
use serde;
use serde_json;

use futures;
use futures::{Future, Stream};

use hyper::header::ContentLength;
use hyper::server::{Request, Response, Service};
use url;

use tunneltimecore::models;

use web::responses;
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
            "/health" => match req.method() {
                hyper::Method::Get => handle_health(),
                _ => return responses::method_not_allowed(req.method()),
            },
            "/api/users" => match req.method() {
                hyper::Method::Get => handle_user_get(req, &conn),
                hyper::Method::Post => handle_user_post(req, conn),
                _ => return responses::method_not_allowed(req.method()),
            },
            "/api/towns" => match req.method() {
                hyper::Method::Get => handle_town(&req, &conn),
                hyper::Method::Post => handle_town_post(req, conn),
                hyper::Method::Put => handle_town_put(req, conn),
                _ => return responses::method_not_allowed(req.method()),
            },
            "/api/towns/store_front" => match req.method() {
                hyper::Method::Get => handle_store_front(req, conn),
                hyper::Method::Put => handle_store_front_put(req, conn),
                _ => return responses::method_not_allowed(req.method()),
            },
            "/api/dwarves" => match req.method() {
                hyper::Method::Get => handle_dwarves(&req, &conn),
                hyper::Method::Post => handle_dwarves_post(req, conn),
                hyper::Method::Put => handle_dwarf_put(req, conn),
                _ => return responses::method_not_allowed(req.method()),
            },
            _ => return responses::path_not_found(req.uri().path()),
        };
        resp
    }
}

fn handle_health() -> types::ResponseFuture {
    build_response(&serde_json::Value::Object(serde_json::Map::new()))
}

fn handle_user_get(req: Request, ds: &datastore::Datastore) -> types::ResponseFuture {
    let user_name: String = rtry!(get_query_param(&req, "user_name"));
    let user = isetry!(ds.get_user(user_name.to_string()));
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

fn handle_town(req: &Request, ds: &datastore::Datastore) -> types::ResponseFuture {
    let user_id: i32 = rtry!(get_query_param(&req, "user_id"));
    build_response(isetry!(ds.get_town(user_id)))
}

fn handle_town_post(req: Request, ds: datastore::Datastore) -> types::ResponseFuture {
    Box::new(req.body().concat2().and_then(move |chunk| {
        let v: models::UserID = isetry!(
            serde_json::from_slice(&chunk).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        );
        let town = isetry!(ds.new_town(v.user_id));
        build_response(town)
    }))
}

fn handle_town_put(req: Request, ds: datastore::Datastore) -> types::ResponseFuture {
    Box::new(req.body().concat2().and_then(move |chunk| {
        let v: models::TownPut = isetry!(
            serde_json::from_slice(&chunk).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        );
        let town = match v.town_action {
            models::TownAction::PurchaseStoreFront => isetry!(ds.purchase_store_front(v.town_id)),
        };
        build_response(town)
    }))
}

fn handle_store_front(req: Request, ds: datastore::Datastore) -> types::ResponseFuture {
    let user_id: i32 = rtry!(get_query_param(&req, "user_id"));
    build_response(isetry!(ds.get_store_front(user_id)))
}

fn handle_store_front_put(req: Request, ds: datastore::Datastore) -> types::ResponseFuture {
    Box::new(req.body().concat2().and_then(move |chunk| {
        let v: models::PurchasePayload = isetry!(
            serde_json::from_slice(&chunk).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        );
        let town = isetry!(ds.purchase_item(v.town_id, v.to_purchase, v.count));
        build_response(town)
    }))
}

fn handle_dwarves(req: &Request, ds: &datastore::Datastore) -> types::ResponseFuture {
    let town_id: i32 = rtry!(get_query_param(&req, "town_id"));
    build_response(isetry!(ds.get_dwarves(town_id)))
}

fn handle_dwarves_post(req: Request, ds: datastore::Datastore) -> types::ResponseFuture {
    Box::new(req.body().concat2().and_then(move |chunk| {
        let v: models::DwarfCreation = isetry!(
            serde_json::from_slice(&chunk).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        );
        let dwarf = isetry!(ds.recruit_dwarf(v.town_id, v.dwarf_name));
        build_response(dwarf)
    }))
}

fn handle_dwarf_put(req: Request, ds: datastore::Datastore) -> types::ResponseFuture {
    Box::new(req.body().concat2().and_then(move |chunk| {
        let v: models::DwarfDigging = isetry!(
            serde_json::from_slice(&chunk).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        );
        match v.action {
            models::DwarfAction::Dig => {
                let dwarf = isetry!(ds.send_dwarf_digging(v.dwarf_id));
                build_response(dwarf)
            }
            models::DwarfAction::Retrieve => {
                let dwarf = isetry!(ds.retrieve_dwarf(v.dwarf_id));
                build_response(dwarf)
            }
        }
    }))
}

fn get_query_param<T: FromStr>(req: &Request, name: &str) -> Result<T, types::ResponseFuture> {
    let params = url::form_urlencoded::parse(req.query().unwrap_or("").as_bytes())
        .into_owned()
        .collect::<HashMap<String, String>>();
    match params.get(name) {
        Some(val) => match val.parse() {
            Ok(v) => Ok(v),
            Err(_) => Err(responses::bad_request("missing user_name")),
        },
        None => Err(responses::bad_request("missing user_name")),
    }
}

fn build_response<T: serde::Serialize>(ser: T) -> types::ResponseFuture {
    let body = isetry!(serde_json::to_string(&ser));
    Box::new(futures::future::ok(
        Response::new()
            .with_header(ContentLength(body.len() as u64))
            .with_body(body),
    ))
}
