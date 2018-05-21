CREATE TABLE dwarf_mine_trips (
    dwarf_id integer NOT NULL UNIQUE REFERENCES dwarves(id),
    mine_id integer NOT NULL references mines(id),
    finish_time timestamp WITH time zone NOT NULL);
