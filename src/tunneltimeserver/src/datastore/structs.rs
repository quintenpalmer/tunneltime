// Composite Structs
#[derive(Debug, FromRow)]
pub struct TownPlus {
    pub town_id: i32,
    pub user_id: i32,
    pub gold: i32,
    pub gem_shop_id: Option<i32>,
    pub storage_id: i32,
    pub storage_level: i32,
    pub storage_current_stone_count: i32,
    pub storage_max_stone_count: i32,
}

#[derive(Debug, FromRow)]
pub struct DwarfPlus {
    pub id: i32,
    pub town_id: i32,
    pub name: String,
    pub past_finish_time: Option<bool>,
}

#[derive(Debug, FromRow)]
pub struct GemPlus {
    pub gem_id: i32,
    pub size: i32,
    pub gem_type_name: String,
}

#[derive(Debug, FromRow)]
pub struct Stone {
    pub stone_count: i32,
}

// Raw Structs

#[derive(Debug, FromRow)]
pub struct User {
    pub id: i32,
    pub user_name: String,
}

#[derive(Debug, FromRow)]
pub struct Dwarf {
    pub id: i32,
    pub town_id: i32,
    pub name: String,
}

#[derive(Debug, FromRow)]
#[allow(dead_code)]
pub struct Town {
    pub id: i32,
    pub user_id: i32,
}

#[derive(Debug, FromRow)]
pub struct Mine {
    pub id: i32,
    pub town_id: i32,
    pub total_stone: i32,
    pub stone_density: i32,
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
