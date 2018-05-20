CREATE TABLE storage_building_levels (
    level integer NOT NULL UNIQUE,
    max_stone_count integer NOT NULL CHECK (max_stone_count > 0));

CREATE TABLE storage_buildings (
    id serial primary key,
    town_id integer NOT NULL REFERENCES towns(id),
    level integer NOT NULL REFERENCES storage_building_levels(level),
    current_stone_count integer NOT NULL CHECK (current_stone_count >= 0));
