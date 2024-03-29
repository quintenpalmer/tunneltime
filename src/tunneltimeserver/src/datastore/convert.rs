use tunneltimecore::models;

use datastore::structs;

impl structs::TownPlus {
    pub fn into_model(
        self,
        gems: Vec<structs::GemPlus>,
        mine: structs::Mine,
        store_front: Option<models::StoreFront>,
    ) -> models::Town {
        let gem_shop = match self.gem_shop_id {
            Some(_) => {
                let gem_shop_gems = gems.into_iter().map(|x| x.into_model()).collect();
                Some(models::GemShop {
                    gems: gem_shop_gems,
                })
            }
            None => None,
        };
        let storage_building = models::StorageBuilding {
            id: self.storage_id,
            level: self.storage_level,
            current_stone_count: self.storage_current_stone_count,
            max_stone_count: self.storage_max_stone_count,
        };
        models::Town {
            id: self.town_id,
            gem_shop: gem_shop,
            storage_building: storage_building,
            mine: mine.into_model(),
            store_front: store_front,
            gold: self.gold,
        }
    }
}

impl structs::Mine {
    pub fn into_model(self) -> models::Mine {
        models::Mine {
            total_stone: self.total_stone,
            stone_density: self.stone_density,
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

impl structs::DwarfPlus {
    pub fn into_model(&self) -> models::Dwarf {
        models::Dwarf {
            id: self.id,
            name: self.name.clone(),
            status: match self.past_finish_time {
                Some(true) => models::DwarfStatus::Returned,
                Some(false) => models::DwarfStatus::Digging,
                None => models::DwarfStatus::Free,
            },
            town_id: self.town_id,
        }
    }
}

impl structs::StoreFront {
    pub fn into_model(
        &self,
        buying: Vec<structs::ItemInStore>,
        selling: Vec<structs::ItemInStore>,
    ) -> models::StoreFront {
        let buying_map = buying.iter().map(|i| (i.into_model(), i.gold)).collect();
        let selling_map = selling.iter().map(|i| (i.into_model(), i.gold)).collect();
        models::StoreFront {
            id: self.id,
            buying: buying_map,
            selling: selling_map,
        }
    }
}

impl structs::ItemInStore {
    fn into_model(&self) -> models::Item {
        match self.item_name.as_str() {
            "stone" => models::Item::Stone,
            _ => panic!("unsupported item: {}", self.item_name),
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
