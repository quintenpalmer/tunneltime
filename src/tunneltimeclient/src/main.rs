extern crate serde;
extern crate serde_json;
extern crate tunneltimecore;

extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::io::{self, Write};

mod client;
mod draw;
mod error;

fn main() {
    match run_app() {
        Ok(()) => println!("exiting cleanly"),
        Err(err) => println!("error: {:?}", err),
    }
}

fn run_app() -> Result<(), error::Error> {
    io::stdout().write(b"enter your user id\n")?;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let town_id = buffer.trim().parse::<i32>()?;
    let town = client::request_town(town_id)?;
    let town_buf = serde_json::to_string_pretty(&town)?;
    let dwarves = client::request_dwarves(town.id)?;
    let dwarves_buf = serde_json::to_string_pretty(&dwarves)?;
    println!("Let's get digging!");
    println!("Your town:");
    println!("{}", town_buf);
    println!("{}", dwarves_buf);
    println!(
        "test: {}",
        draw::draw_table(vec![draw::Row {
            header: "name".to_string(),
            columns: vec![]
        }])
    );
    return Ok(());
}
