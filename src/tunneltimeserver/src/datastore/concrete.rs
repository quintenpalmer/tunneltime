use postgres;
use postgres_extra;

use tunneltimecore::models;

use datastore::queries;
use datastore::structs;
use error;

pub struct Datastore {
    conn: postgres::Connection,
}

impl Datastore {
    pub fn new(
        host: String,
        dbname: &str,
        port: u16,
        username: &str,
        password: Option<&str>,
    ) -> Result<Datastore, error::Error> {
        let connect_params = postgres::params::Builder::new()
            .database(dbname)
            .port(port)
            .user(username, password)
            .build(postgres::params::Host::Tcp(host));
        let conn = postgres::Connection::connect(connect_params, postgres::TlsMode::None)?;
        return Ok(Datastore { conn: conn });
    }

    pub fn get_town(&self, user_id: i32) -> Result<models::Town, error::Error> {
        let town: structs::TownPlus = select_one_by_field(
            self,
            "towns".to_string(),
            queries::TOWN_BY_USER_ID_SQL,
            user_id,
        )?;
        let gems = {
            match town.gem_shop_id {
                Some(gem_shop_id) => {
                    select_by_field(self, queries::GEMS_BY_GEM_SHOP_ID_SQL, gem_shop_id)?
                }
                None => Vec::new(),
            }
        };
        Ok(town.into_model(gems))
    }
}

pub fn select_one_by_field<T, F>(
    ds: &Datastore,
    name: String,
    query: &'static str,
    id: F,
) -> Result<T, error::Error>
where
    T: postgres_extra::FromRow,
    F: postgres::types::ToSql,
{
    let rows = ds.conn.query(query, &[&id])?;
    if rows.len() != 1 {
        return Err(error::Error::SelectManyOnOne(name));
    }
    let row = rows.get(0);
    let ret = T::parse_row(row)?;
    Ok(ret)
}

pub fn select_by_field<T, F>(
    ds: &Datastore,
    query: &'static str,
    id: F,
) -> Result<Vec<T>, error::Error>
where
    T: postgres_extra::FromRow,
    F: postgres::types::ToSql,
{
    let rows = ds.conn.query(query, &[&id])?;
    let mut ret = Vec::new();
    for row in rows.iter() {
        ret.push(T::parse_row(row)?);
    }
    return Ok(ret);
}

impl structs::TownPlus {
    fn into_model(self, gems: Vec<structs::GemPlus>) -> models::Town {
        let gem_shop = match self.gem_shop_id {
            Some(_) => {
                let gem_shop_gems = gems.into_iter().map(|x| x.into_model()).collect();
                Some(models::GemShop {
                    gems: gem_shop_gems,
                })
            }
            None => None,
        };
        models::Town { gem_shop: gem_shop }
    }
}

impl structs::GemPlus {
    fn into_model(self) -> models::Gem {
        models::Gem {
            type_: match self.gem_type_name.as_str() {
                "emerald" => models::GemType::Emerald,
                "ruby" => models::GemType::Ruby,
                "sapphire" => models::GemType::Sapphire,
                _ => panic!("unsupported gem type: {}", self.gem_type_name),
            },
            size: self.size as u32,
        }
    }
}
