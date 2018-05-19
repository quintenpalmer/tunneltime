#[derive(Debug, Serialize, Deserialize)]
pub struct Town {
    pub gem_shop: Option<GemShop>,
}

impl Town {
    pub fn new() -> Town {
        Town { gem_shop: None }
    }

    pub fn acquire_gem_shop(&mut self) {
        self.gem_shop = Some(GemShop {
            gems: vec![
                Gem {
                    type_: GemType::Ruby,
                    size: 1,
                },
            ],
        });
    }
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
