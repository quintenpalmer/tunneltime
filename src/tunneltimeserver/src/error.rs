use hyper;
use postgres;
use postgres_extra;
use serde_json;
use std::io;

#[derive(Debug)]
pub enum Error {
    SerdeJSON(serde_json::Error),
    IO(io::Error),
    HyperURI(hyper::error::UriError),
    Hyper(hyper::Error),
    Postgres(postgres::error::Error),
    PostgresExtra(postgres_extra::ExtraError),
    SelectManyOnOne(String),
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

impl From<postgres::error::Error> for Error {
    fn from(error: postgres::error::Error) -> Self {
        Error::Postgres(error)
    }
}

impl From<postgres_extra::Error> for Error {
    fn from(error: postgres_extra::Error) -> Self {
        match error {
            postgres_extra::Error::Postgres(pg_error) => Error::Postgres(pg_error),
            postgres_extra::Error::Extra(extra_error) => Error::PostgresExtra(extra_error),
        }
    }
}
