extern crate serde;
extern crate serde_json;
extern crate tunneltimecore;

extern crate futures;
extern crate hyper;
extern crate tokio_core;

mod client;
mod error;

fn main() {
    match run_app() {
        Ok(()) => println!("exiting cleanly"),
        Err(err) => println!("error: {:?}", err),
    }
}

fn run_app() -> Result<(), error::Error> {
    let town = client::request_town()?;
    let buf = serde_json::to_string_pretty(&town)?;
    println!("Let's get digging!");
    println!("Your town:");
    println!("{}", buf);
    return Ok(());
}
