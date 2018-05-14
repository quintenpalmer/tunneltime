extern crate postgres;
extern crate serde;
extern crate serde_json;

extern crate futures;
extern crate hyper;

extern crate tunneltimecore;

extern crate postgres_extra;
#[macro_use]
extern crate postgres_extra_derive;

use std::net;

mod datastore;
mod error;
mod server;

fn main() {
    match run_app() {
        Ok(()) => println!("exiting cleanly"),
        Err(err) => println!("error: {:?}", err),
    }
}

fn run_app() -> Result<(), error::Error> {
    let port = 5269;
    let addr = net::SocketAddr::new(net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1)), port);
    let server = hyper::server::Http::new().bind(&addr, || Ok(server::Handler {}))?;
    println!("running on port: {}", port);
    server.run()?;
    Ok(())
}
