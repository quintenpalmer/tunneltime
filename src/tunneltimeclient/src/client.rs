use futures::{Future, Stream};
use hyper;
use serde;
use serde_json;
use std::io;
use tokio_core::reactor::Core;

use error;
use tunneltimecore::models;

pub fn request_town(user_id: i32) -> Result<models::Town, error::Error> {
    return request(format!("http://localhost:5269/api/towns?user_id={}", user_id).as_str());
}

pub fn request_dwarves(town_id: i32) -> Result<Vec<models::Dwarf>, error::Error> {
    return request(format!("http://localhost:5269/api/dwarves?town_id={}", town_id).as_str());
}

fn request<T: serde::de::DeserializeOwned>(uri_str: &str) -> Result<T, error::Error> {
    let mut core = Core::new()?;
    let client = hyper::Client::new(&core.handle());
    let uri = uri_str.parse()?;

    let work = client.get(uri).and_then(|res| {
        res.body().concat2().and_then(move |chunk| {
            let v = serde_json::from_slice(&chunk)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            Ok(v)
        })
    });
    let v = core.run(work)?;
    return Ok(v);
}
