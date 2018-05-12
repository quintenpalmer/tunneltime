use postgres;

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
        let town = {
            let raw_rows = self.conn.query(queries::TOWN_BY_USER_ID_SQL, &[&user_id])?;
            let rows: Vec<postgres::rows::Row> = raw_rows.into_iter().collect();
            let town = match rows.as_slice() {
                [row] => Ok(structs::TownPlus {
                    town_id: row.get(0),
                    user_id: row.get(1),
                    gem_shop_id: row.get(2),
                }),
                _ => Err(error::Error::SelectManyOnOne("towns".to_string())),
            }?;
            town
        };
        let gems = {
            match town.gem_shop_id {
                Some(gem_shop_id) => {
                    let mut gems = Vec::new();
                    for row in self.conn
                        .query(queries::GEMS_BY_GEM_SHOP_ID_SQL, &[&gem_shop_id])?
                        .iter()
                    {
                        gems.push(structs::GemPlus {
                            gem_id: row.get(0),
                            size: row.get(1),
                            gem_type_name: row.get(2),
                        });
                    }
                    gems
                }
                None => Vec::new(),
            }
        };
        Ok(town.into_model(gems))
    }
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
