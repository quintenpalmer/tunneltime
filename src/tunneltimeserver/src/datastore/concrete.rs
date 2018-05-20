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

    pub fn new_town(&self, user_id: i32) -> Result<models::Town, error::Error> {
        let _ = self.conn.execute(queries::INSERT_TOWN, &[&user_id])?;
        self.get_town(user_id)
    }

    pub fn get_town(&self, user_id: i32) -> Result<models::Town, error::Error> {
        let town: structs::TownPlus = select_one_by_field(
            &self.conn,
            "towns".to_string(),
            queries::TOWN_BY_USER_ID_SQL,
            user_id,
        )?;
        let gems = {
            match town.gem_shop_id {
                Some(gem_shop_id) => {
                    select_by_field(&self.conn, queries::GEMS_BY_GEM_SHOP_ID_SQL, gem_shop_id)?
                }
                None => Vec::new(),
            }
        };
        Ok(town.into_model(gems))
    }

    pub fn recruit_dwarf(
        &self,
        town_id: i32,
        dwarf_name: String,
    ) -> Result<Vec<models::Dwarf>, error::Error> {
        let txn = self.conn.transaction()?;
        let _ = txn.execute(queries::INSERT_DWARF, &[&town_id, &dwarf_name])?;
        let _ = txn.execute(queries::UPDATE_TOWN_GOLD, &[&town_id])?;
        let dwarves: Vec<structs::Dwarf> =
            select_by_field(&txn, queries::DWARVES_BY_TOWN_ID, town_id)?;
        txn.set_commit();
        Ok(dwarves.into_iter().map(|x| x.into_model()).collect())
    }

    pub fn get_dwarves(&self, town_id: i32) -> Result<Vec<models::Dwarf>, error::Error> {
        let dwarves: Vec<structs::Dwarf> =
            select_by_field(&self.conn, queries::DWARVES_BY_TOWN_ID, town_id)?;
        Ok(dwarves.into_iter().map(|x| x.into_model()).collect())
    }

    pub fn get_user(&self, user_name: String) -> Result<models::User, error::Error> {
        let user: structs::User = select_one_by_field(
            &self.conn,
            "users".to_string(),
            queries::USER_BY_USER_NAME,
            user_name,
        )?;
        Ok(user.into_model())
    }

    pub fn new_user(&self, user_name: String) -> Result<models::User, error::Error> {
        let _ = self.conn.execute(queries::INSERT_USER, &[&user_name])?;
        self.get_user(user_name)
    }
}

pub fn select_one_by_field<T, F>(
    ds: &postgres::GenericConnection,
    name: String,
    query: &'static str,
    id: F,
) -> Result<T, error::Error>
where
    T: postgres_extra::FromRow,
    F: postgres::types::ToSql,
{
    let rows = ds.query(query, &[&id])?;
    if rows.len() != 1 {
        return Err(error::Error::SelectManyOnOne(name));
    }
    let row = rows.get(0);
    let ret = T::parse_row(row)?;
    Ok(ret)
}

pub fn select_by_field<T, F>(
    ds: &postgres::GenericConnection,
    query: &'static str,
    id: F,
) -> Result<Vec<T>, error::Error>
where
    T: postgres_extra::FromRow,
    F: postgres::types::ToSql,
{
    let rows = ds.query(query, &[&id])?;
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
        models::Town {
            id: self.town_id,
            gem_shop: gem_shop,
            gold: self.gold,
        }
    }
}

impl structs::GemPlus {
    fn into_model(self) -> models::Gem {
        models::Gem {
            id: self.gem_id,
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

impl structs::Dwarf {
    fn into_model(self) -> models::Dwarf {
        models::Dwarf {
            id: self.id,
            name: self.name,
        }
    }
}

impl structs::User {
    fn into_model(self) -> models::User {
        models::User {
            id: self.id,
            user_name: self.user_name,
        }
    }
}
