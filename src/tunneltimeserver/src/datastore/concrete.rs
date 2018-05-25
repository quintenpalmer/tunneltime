use postgres as pg;

use tunneltimecore::models;

use datastore::queries;
use datastore::selects;
use datastore::structs;
use error;

pub struct Datastore {
    conn: pg::Connection,
}

impl Datastore {
    pub fn new(
        host: String,
        dbname: &str,
        port: u16,
        username: &str,
        password: Option<&str>,
    ) -> Result<Datastore, error::Error> {
        let connect_params = pg::params::Builder::new()
            .database(dbname)
            .port(port)
            .user(username, password)
            .build(pg::params::Host::Tcp(host));
        let conn = pg::Connection::connect(connect_params, pg::TlsMode::None)?;
        return Ok(Datastore { conn: conn });
    }

    pub fn new_town(&self, user_id: i32) -> Result<models::Town, error::Error> {
        let txn = self.conn.transaction()?;
        txn.execute(queries::INSERT_TOWN, &[&user_id])?;
        let simple_town: structs::Town = selects::select_one_by_field(
            &txn,
            "towns".to_string(),
            queries::SIMPLE_TOWN_BY_USER_ID,
            user_id,
        )?;
        txn.execute(queries::INSERT_NEW_STORAGE_BUILDING, &[&simple_town.id])?;
        txn.execute(queries::INSERT_NEW_MINE, &[&simple_town.id])?;
        txn.set_commit();
        return get_town(&txn, user_id);
    }

    pub fn get_town(&self, user_id: i32) -> Result<models::Town, error::Error> {
        return get_town(&self.conn, user_id);
    }

    pub fn recruit_dwarf(
        &self,
        town_id: i32,
        dwarf_name: String,
    ) -> Result<Vec<models::Dwarf>, error::Error> {
        let txn = self.conn.transaction()?;
        txn.execute(queries::INSERT_DWARF, &[&town_id, &dwarf_name])?;
        txn.execute(queries::UPDATE_TOWN_GOLD, &[&town_id])?;
        let dwarves = get_dwarves(&txn, town_id)?;
        txn.set_commit();
        Ok(dwarves)
    }

    pub fn get_dwarves(&self, town_id: i32) -> Result<Vec<models::Dwarf>, error::Error> {
        get_dwarves(&self.conn, town_id)
    }

    pub fn get_user(&self, user_name: String) -> Result<models::User, error::Error> {
        let user: structs::User = selects::select_one_by_field(
            &self.conn,
            "users".to_string(),
            queries::USER_BY_USER_NAME,
            user_name,
        )?;
        Ok(user.into_model())
    }

    pub fn new_user(&self, user_name: String) -> Result<models::User, error::Error> {
        self.conn.execute(queries::INSERT_USER, &[&user_name])?;
        self.get_user(user_name)
    }

    pub fn send_dwarf_digging(&self, dwarf_id: i32) -> Result<models::Dwarf, error::Error> {
        let txn = self.conn.transaction()?;
        let dwarf = get_dwarf(&txn, dwarf_id)?;
        match dwarf.status {
            models::DwarfStatus::Free => (),
            models::DwarfStatus::Digging => return Err(error::Error::DwarfBusy(dwarf_id)),
            models::DwarfStatus::Returned => return Err(error::Error::DwarfBusy(dwarf_id)),
        };
        let mine = get_mine(&txn, dwarf.town_id)?;
        txn.execute(queries::SEND_DWARF_DIGGING, &[&dwarf.id, &mine.id])?;
        txn.execute(queries::MARK_MINE_STONE_LOSS, &[&mine.id])?;
        let dwarf2 = get_dwarf(&txn, dwarf_id)?;
        txn.set_commit();
        Ok(dwarf2)
    }

    pub fn retrieve_dwarf(&self, dwarf_id: i32) -> Result<models::Dwarf, error::Error> {
        let txn = self.conn.transaction()?;
        let dwarf = get_dwarf(&txn, dwarf_id)?;
        match dwarf.status {
            models::DwarfStatus::Free => return Err(error::Error::DwarfNotReturned(dwarf_id)),
            models::DwarfStatus::Digging => return Err(error::Error::DwarfNotReturned(dwarf_id)),
            models::DwarfStatus::Returned => (),
        };
        let mine = get_mine(&txn, dwarf.town_id)?;
        txn.execute(
            queries::UPDATE_STONE_STORAGE,
            &[&dwarf.town_id, &mine.stone_density],
        )?;
        txn.execute(queries::RETRIEVE_DWARF_DIGGING, &[&dwarf_id])?;
        let dwarf2 = get_dwarf(&txn, dwarf_id)?;
        txn.set_commit();
        Ok(dwarf2)
    }
}

fn get_town(ds: &pg::GenericConnection, user_id: i32) -> Result<models::Town, error::Error> {
    let town: structs::TownPlus = selects::select_one_by_field(
        ds,
        "towns".to_string(),
        queries::TOWN_BY_USER_ID_SQL,
        user_id,
    )?;
    let gems = {
        match town.gem_shop_id {
            Some(gem_shop_id) => {
                selects::select_by_field(ds, queries::GEMS_BY_GEM_SHOP_ID_SQL, gem_shop_id)?
            }
            None => Vec::new(),
        }
    };
    let mine = get_mine(ds, town.town_id)?;

    Ok(town.into_model(gems, mine))
}

fn get_mine(ds: &pg::GenericConnection, town_id: i32) -> Result<structs::Mine, error::Error> {
    let mine: structs::Mine =
        selects::select_one_by_field(ds, "mines".to_string(), queries::MINES_BY_TOWN_ID, town_id)?;
    Ok(mine)
}

fn get_dwarves(
    ds: &pg::GenericConnection,
    town_id: i32,
) -> Result<Vec<models::Dwarf>, error::Error> {
    let dwarves: Vec<structs::DwarfPlus> =
        selects::select_by_field(ds, queries::DWARVES_BY_TOWN_ID, town_id)?;
    Ok(dwarves.into_iter().map(|x| x.into_model()).collect())
}

fn get_dwarf(ds: &pg::GenericConnection, dwarf_id: i32) -> Result<models::Dwarf, error::Error> {
    let dwarf: structs::DwarfPlus =
        selects::select_one_by_field(ds, "dwarves".to_string(), queries::DWARF_BY_ID, dwarf_id)?;
    let model_dwarf = dwarf.into_model();
    Ok(model_dwarf)
}
