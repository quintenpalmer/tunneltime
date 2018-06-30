use postgres as pg;
use postgres_extra as pg_extra;

use error;

pub fn select_one_by_field<T, F>(
    ds: &pg::GenericConnection,
    name: String,
    query: &'static str,
    id: F,
) -> Result<T, error::Error>
where
    T: pg_extra::FromRow,
    F: pg::types::ToSql,
{
    let rows = ds.query(query, &[&id])?;
    if rows.len() != 1 {
        return Err(error::Error::SelectManyOnOne(name));
    }
    let row = rows.get(0);
    let ret = T::parse_row(row)?;
    Ok(ret)
}

pub fn select_maybe_one_by_field<T, F>(
    ds: &pg::GenericConnection,
    name: String,
    query: &'static str,
    id: F,
) -> Result<Option<T>, error::Error>
where
    T: pg_extra::FromRow,
    F: pg::types::ToSql,
{
    let rows = ds.query(query, &[&id])?;
    match rows.len() {
        0 => Ok(None),
        1 => {
            let row = rows.get(0);
            let ret = T::parse_row(row)?;
            Ok(Some(ret))
        }
        _ => Err(error::Error::SelectManyOnOne(name)),
    }
}

pub fn select_by_field<T, F>(
    ds: &pg::GenericConnection,
    query: &'static str,
    id: F,
) -> Result<Vec<T>, error::Error>
where
    T: pg_extra::FromRow,
    F: pg::types::ToSql,
{
    let rows = ds.query(query, &[&id])?;
    let mut ret = Vec::new();
    for row in rows.iter() {
        ret.push(T::parse_row(row)?);
    }
    return Ok(ret);
}
