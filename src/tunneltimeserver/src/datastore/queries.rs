pub static INSERT_TOWN: &'static str = r#"
    INSERT INTO
        towns (user_id, gold)
    VALUES
        ($1, 100)
"#;

pub static SIMPLE_TOWN_BY_USER_ID: &'static str = r#"
    SELECT
        towns.id,
        towns.user_id,
        towns.gold
    FROM
        towns
    WHERE
        user_id = $1
"#;

macro_rules! town_base_sql {
    () => ( r#"
    SELECT
        towns.id as town_id,
        towns.user_id,
        towns.gold,
        gem_shops.id as gem_shop_id,
        storage_buildings.id as storage_id,
        storage_buildings.level as storage_level,
        storage_buildings.current_stone_count as storage_current_stone_count,
        storage_building_levels.max_stone_count as storage_max_stone_count
    FROM
        towns
    LEFT JOIN
        gem_shops ON gem_shops.town_id = towns.id
    INNER JOIN
        storage_buildings ON storage_buildings.town_id = towns.id
    INNER JOIN
        storage_building_levels ON storage_buildings.level = storage_building_levels.level
"# )
}

pub static TOWN_BY_USER_ID_SQL: &'static str = concat!(town_base_sql!(), "WHERE user_id = $1");

pub static TOWN_BY_TOWN_ID_SQL: &'static str = concat!(town_base_sql!(), "WHERE towns.id = $1");

pub static MINES_BY_TOWN_ID: &'static str = r#"
    SELECT
        mines.id,
        mines.town_id,
        mines.total_stone,
        mines.stone_density
    FROM
        mines
    WHERE
        mines.town_id = $1
"#;

pub static UPDATE_TOWN_GOLD: &'static str = r#"
    UPDATE
        towns
    SET
        gold=towns.gold-20
    WHERE
        id=$1
"#;

pub static GEMS_BY_GEM_SHOP_ID_SQL: &'static str = r#"
    SELECT
        gems.id as gem_id,
        gems.size,
        gem_types.name as gem_type_name
    FROM
        gems
    INNER JOIN
        gem_types ON gem_types.id = gems.gem_type_id
    INNER JOIN
        gem_shop_gems ON gem_shop_gems.gem_id = gems.id
    WHERE
        gem_shop_gems.gem_shop_id = $1
"#;

pub static UPDATE_STONE_STORAGE: &'static str = r#"
    UPDATE
        storage_buildings
    SET
        current_stone_count=(current_stone_count + $2)
    WHERE
        storage_buildings.town_id = $1
"#;

pub static DWARVES_BY_TOWN_ID: &'static str = r#"
	SELECT
        dwarves.id,
        dwarves.town_id,
        dwarves.name,
        dwarf_mine_trips.finish_time < now() as past_finish_time
    FROM
        dwarves
    LEFT JOIN
        dwarf_mine_trips ON dwarf_mine_trips.dwarf_id = dwarves.id
    WHERE
        dwarves.town_id = $1
"#;

pub static DWARF_BY_ID: &'static str = r#"
	SELECT
        dwarves.id,
        dwarves.town_id,
        dwarves.name,
        dwarf_mine_trips.finish_time < now() as past_finish_time
    FROM
        dwarves
    LEFT JOIN
        dwarf_mine_trips ON dwarf_mine_trips.dwarf_id = dwarves.id
    WHERE
        dwarves.id = $1
"#;

pub static INSERT_STORE_FRONT: &'static str = r#"
    INSERT INTO
        store_fronts (town_id)
    VALUES
        ($1)
"#;

pub static INSERT_STORE_FRONT_BUYING: &'static str = r#"
    INSERT INTO
        store_front_buying_items (store_front_id, item_id, gold)
    VALUES
        ($1, $2, $3)
"#;

pub static INSERT_STORE_FRONT_SELLING: &'static str = r#"
    INSERT INTO
        store_front_selling_items (store_front_id, item_id, gold)
    VALUES
        ($1, $2, $3)
"#;

pub static INSERT_USER: &'static str = r#"
    INSERT INTO
        users (user_name)
    VALUES
        ($1)
"#;

pub static INSERT_DWARF: &'static str = r#"
    INSERT INTO
        dwarves (town_id, name)
    VALUES
        ($1, $2)
"#;

pub static INSERT_NEW_STORAGE_BUILDING: &'static str = r#"
    INSERT INTO
        storage_buildings (town_id, level, current_stone_count)
    VALUES
        ($1, 1, 0);
"#;

pub static INSERT_NEW_MINE: &'static str = r#"
    INSERT INTO
        mines (town_id, total_stone, stone_density)
    VALUES ($1, 1000, 5);
"#;

pub static USER_BY_USER_NAME: &'static str = r#"
    SELECT
        users.id,
        users.user_name
    FROM
        users
    WHERE
        users.user_name = $1
"#;

pub static SEND_DWARF_DIGGING: &'static str = r#"
    INSERT INTO
        dwarf_mine_trips (dwarf_id, mine_id, finish_time)
    VALUES ($1, $2, now() + '1 minute')
"#;

pub static RETRIEVE_DWARF_DIGGING: &'static str = r#"
	DELETE FROM
        dwarf_mine_trips
    WHERE
        dwarf_id = $1
"#;

pub static MARK_MINE_STONE_LOSS: &'static str = r#"
    UPDATE
        mines
    SET
        total_stone = (total_stone - stone_density)
    WHERE
        id = $1
"#;

pub static GET_STORE_FRONT_BY_TOWN_ID: &'static str = r#"
    SELECT
        id,
        town_id
    FROM
        store_fronts
    WHERE
        town_id = $1
"#;

pub static GET_STORE_BUYING_ITEMS: &'static str = r#"
    SELECT
        sfbi.store_front_id,
        sfbi.item_id,
        items.name as item_name,
        sfbi.gold
    FROM
        store_front_buying_items sfbi
    INNER JOIN
        items on items.id = sfbi.item_id
    WHERE
        sfbi.store_front_id = $1
"#;

pub static GET_STORE_SELLING_ITEMS: &'static str = r#"
    SELECT
        sfsi.store_front_id,
        sfsi.item_id,
        items.name as item_name,
        sfsi.gold
    FROM
        store_front_selling_items sfsi
    INNER JOIN
        items on items.id = sfsi.item_id
    WHERE
        sfsi.store_front_id = $1
"#;
