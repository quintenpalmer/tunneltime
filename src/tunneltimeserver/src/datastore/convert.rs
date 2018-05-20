use tunneltimecore::models;

use datastore::structs;

impl structs::TownPlus {
    pub fn into_model(self, gems: Vec<structs::GemPlus>) -> models::Town {
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
    pub fn into_model(self) -> models::Gem {
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
    pub fn into_model(self) -> models::Dwarf {
        models::Dwarf {
            id: self.id,
            name: self.name,
        }
    }
}

impl structs::User {
    pub fn into_model(self) -> models::User {
        models::User {
            id: self.id,
            user_name: self.user_name,
        }
    }
}
