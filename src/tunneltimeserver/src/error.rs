use hyper;
use postgres as pg;
use postgres_extra as pg_extra;
use serde_json;
use std::io;

#[derive(Debug)]
pub enum Error {
    SerdeJSON(serde_json::Error),
    IO(io::Error),
    HyperURI(hyper::error::UriError),
    Hyper(hyper::Error),
    Postgres(pg::error::Error),
    PostgresExtra(pg_extra::ExtraError),
    NoSqlRows,
    SelectManyOnOne(String),
    DwarfBusy(i32),
    DwarfNotReturned(i32),
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::SerdeJSON(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IO(error)
    }
}

impl From<hyper::Error> for Error {
    fn from(error: hyper::Error) -> Self {
        Error::Hyper(error)
    }
}

impl From<hyper::error::UriError> for Error {
    fn from(error: hyper::error::UriError) -> Self {
        Error::HyperURI(error)
    }
}

impl From<pg::error::Error> for Error {
    fn from(error: pg::error::Error) -> Self {
        Error::Postgres(error)
    }
}

impl From<pg_extra::Error> for Error {
    fn from(error: pg_extra::Error) -> Self {
        match error {
            pg_extra::Error::Postgres(pg_error) => Error::Postgres(pg_error),
            pg_extra::Error::Extra(extra_error) => Error::PostgresExtra(extra_error),
        }
    }
}
