// Composite Structs
#[derive(Debug, FromRow)]
pub struct TownPlus {
    pub town_id: i32,
    pub user_id: i32,
    pub gold: Option<i32>,
    pub gem_shop_id: Option<i32>,
}

#[derive(Debug, FromRow)]
pub struct GemPlus {
    pub gem_id: i32,
    pub size: i32,
    pub gem_type_name: String,
}

// Raw Structs

#[derive(Debug, FromRow)]
pub struct Dwarf {
    pub id: i32,
    pub town_id: i32,
    pub name: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Town {
    pub id: i32,
    pub user_id: i32,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct GemShop {
    pub id: i32,
    pub town_id: i32,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct GemShopGems {
    pub gem_shop_id: i32,
    pub gem_id: i32,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Gem {
    pub id: i32,
    pub gem_type_id: i32,
    pub size: i32,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct GemType {
    pub id: i32,
    pub name: String,
}
