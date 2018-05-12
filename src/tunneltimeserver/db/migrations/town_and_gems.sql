CREATE TABLE towns(id serial PRIMARY KEY, user_id integer NOT NULL UNIQUE);

CREATE TABLE gem_shops(id serial PRIMARY KEY, town_id integer NOT NULL REFERENCES towns(id));

CREATE TABLE gem_types(id serial PRIMARY KEY, name text NOT NULL UNIQUE);

CREATE TABLE gems(id serial PRIMARY KEY, gem_type_id integer NOT NULL REFERENCES gem_types(id), size integer NOT NULL);

CREATE TABLE gem_shop_gems(gem_shop_id integer NOT NULL REFERENCES gem_shops(id), gem_id integer NOT NULL REFERENCES gems(id));
