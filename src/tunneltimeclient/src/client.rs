use futures::{Future, Stream};
use hyper;
use serde_json;
use std::io;
use tokio_core::reactor::Core;

use error;
use tunneltimecore::models;

pub fn request_town() -> Result<models::Town, error::Error> {
    let uri_str = "http://localhost:5269/api/towns";
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
