use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub user_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Town {
    pub id: i32,
    pub gem_shop: Option<GemShop>,
    pub store_front: Option<StoreFront>,
    pub storage_building: StorageBuilding,
    pub mine: Mine,
    pub gold: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GemShop {
    pub gems: Vec<Gem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageBuilding {
    pub id: i32,
    pub level: i32,
    pub current_stone_count: i32,
    pub max_stone_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mine {
    pub total_stone: i32,
    pub stone_density: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreFront {
    pub id: i32,
    pub selling: BTreeMap<Item, i32>,
    pub buying: BTreeMap<Item, i32>,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum Item {
    Stone,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Gem {
    pub id: i32,
    pub type_: GemType,
    pub size: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GemType {
    Ruby,
    Sapphire,
    Emerald,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DwarfStatus {
    Free,
    Digging,
    Returned,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dwarf {
    pub id: i32,
    pub name: String,
    pub status: DwarfStatus,
    pub town_id: i32,
}

// POST bodies

#[derive(Debug, Serialize, Deserialize)]
pub struct UserID {
    pub user_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DwarfCreation {
    pub town_id: i32,
    pub dwarf_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub user_name: String,
}

// PUT bodies

#[derive(Debug, Serialize, Deserialize)]
pub struct DwarfDigging {
    pub dwarf_id: i32,
    pub action: DwarfAction,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DwarfAction {
    Dig,
    Retrieve,
}
