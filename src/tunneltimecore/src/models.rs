#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub user_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Town {
    pub gem_shop: Option<GemShop>,
    pub gold: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GemShop {
    pub gems: Vec<Gem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Gem {
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
pub struct Dwarf {
    pub name: String,
}
