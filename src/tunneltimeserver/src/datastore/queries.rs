pub static TOWN_BY_USER_ID_SQL: &'static str = r#"
    SELECT
        towns.id as town_id,
        towns.user_id,
        towns.gold,
        gem_shops.id as gem_shop_id
    FROM
        towns
    INNER JOIN
        gem_shops ON gem_shops.town_id = towns.id
    WHERE
        user_id = $1
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

pub static DWARVES_BY_TOWN_ID: &'static str = r#"
	SELECT
        dwarves.id,
        dwarves.town_id,
        dwarves.name
    FROM
        dwarves
    WHERE
        dwarves.town_id = $1
"#;
