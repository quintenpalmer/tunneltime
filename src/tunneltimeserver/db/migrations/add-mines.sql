CREATE TABLE mines (
    id serial PRIMARY KEY,
    town_id integer NOT NULL REFERENCES towns(id),
    total_stone integer NOT NULL CHECK (total_stone >= 0),
    stone_density integer NOT NULL CHECK (stone_density > 0));

