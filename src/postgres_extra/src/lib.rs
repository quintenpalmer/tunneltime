extern crate postgres;

#[derive(Debug)]
pub enum Error {
    Postgres(postgres::error::Error),
    Extra(ExtraError),
}

#[derive(Debug)]
pub enum ExtraError {
    ParseNonexistentField(String),
}

pub trait FromRow {
    fn parse_row(row: postgres::rows::Row) -> Result<Self, Error>
    where
        Self: Sized;
}
