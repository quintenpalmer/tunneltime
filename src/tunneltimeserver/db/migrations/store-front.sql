CREATE TABLE items(
    id serial primary key,
    name text NOT NULL UNIQUE);

CREATE TABLE store_fronts(
    id serial primary key,
    town_id integer NOT NULL REFERENCES towns(id) UNIQUE);

CREATE TABLE store_front_selling_items(
    store_front_id integer NOT NULL REFERENCES store_fronts(id),
    item_id integer NOT NULL REFERENCES items(id),
    gold integer NOT NULL,
	UNIQUE (store_front_id, item_id));

CREATE TABLE store_front_buying_items(
    store_front_id integer NOT NULL REFERENCES store_fronts(id),
    item_id integer NOT NULL REFERENCES items(id),
    gold integer NOT NULL,
	UNIQUE (store_front_id, item_id));
