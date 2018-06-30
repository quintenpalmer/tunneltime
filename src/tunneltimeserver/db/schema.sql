--
-- PostgreSQL database dump
--

-- Dumped from database version 9.6.8
-- Dumped by pg_dump version 9.6.8

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: plpgsql; Type: EXTENSION; Schema: -; Owner: 
--

CREATE EXTENSION IF NOT EXISTS plpgsql WITH SCHEMA pg_catalog;


--
-- Name: EXTENSION plpgsql; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION plpgsql IS 'PL/pgSQL procedural language';


SET default_tablespace = '';

SET default_with_oids = false;

--
-- Name: dwarf_mine_trips; Type: TABLE; Schema: public; Owner: tunneltime_user
--

CREATE TABLE public.dwarf_mine_trips (
    dwarf_id integer NOT NULL,
    mine_id integer NOT NULL,
    finish_time timestamp with time zone NOT NULL
);


ALTER TABLE public.dwarf_mine_trips OWNER TO tunneltime_user;

--
-- Name: dwarves; Type: TABLE; Schema: public; Owner: tunneltime_user
--

CREATE TABLE public.dwarves (
    id integer NOT NULL,
    town_id integer NOT NULL,
    name text NOT NULL
);


ALTER TABLE public.dwarves OWNER TO tunneltime_user;

--
-- Name: dwarves_id_seq; Type: SEQUENCE; Schema: public; Owner: tunneltime_user
--

CREATE SEQUENCE public.dwarves_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.dwarves_id_seq OWNER TO tunneltime_user;

--
-- Name: dwarves_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: tunneltime_user
--

ALTER SEQUENCE public.dwarves_id_seq OWNED BY public.dwarves.id;


--
-- Name: gem_shop_gems; Type: TABLE; Schema: public; Owner: tunneltime_user
--

CREATE TABLE public.gem_shop_gems (
    gem_shop_id integer NOT NULL,
    gem_id integer NOT NULL
);


ALTER TABLE public.gem_shop_gems OWNER TO tunneltime_user;

--
-- Name: gem_shops; Type: TABLE; Schema: public; Owner: tunneltime_user
--

CREATE TABLE public.gem_shops (
    id integer NOT NULL,
    town_id integer NOT NULL
);


ALTER TABLE public.gem_shops OWNER TO tunneltime_user;

--
-- Name: gem_shops_id_seq; Type: SEQUENCE; Schema: public; Owner: tunneltime_user
--

CREATE SEQUENCE public.gem_shops_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.gem_shops_id_seq OWNER TO tunneltime_user;

--
-- Name: gem_shops_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: tunneltime_user
--

ALTER SEQUENCE public.gem_shops_id_seq OWNED BY public.gem_shops.id;


--
-- Name: gem_types; Type: TABLE; Schema: public; Owner: tunneltime_user
--

CREATE TABLE public.gem_types (
    id integer NOT NULL,
    name text NOT NULL
);


ALTER TABLE public.gem_types OWNER TO tunneltime_user;

--
-- Name: gem_types_id_seq; Type: SEQUENCE; Schema: public; Owner: tunneltime_user
--

CREATE SEQUENCE public.gem_types_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.gem_types_id_seq OWNER TO tunneltime_user;

--
-- Name: gem_types_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: tunneltime_user
--

ALTER SEQUENCE public.gem_types_id_seq OWNED BY public.gem_types.id;


--
-- Name: gems; Type: TABLE; Schema: public; Owner: tunneltime_user
--

CREATE TABLE public.gems (
    id integer NOT NULL,
    gem_type_id integer NOT NULL,
    size integer NOT NULL
);


ALTER TABLE public.gems OWNER TO tunneltime_user;

--
-- Name: gems_id_seq; Type: SEQUENCE; Schema: public; Owner: tunneltime_user
--

CREATE SEQUENCE public.gems_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.gems_id_seq OWNER TO tunneltime_user;

--
-- Name: gems_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: tunneltime_user
--

ALTER SEQUENCE public.gems_id_seq OWNED BY public.gems.id;


--
-- Name: items; Type: TABLE; Schema: public; Owner: tunneltime_user
--

CREATE TABLE public.items (
    id integer NOT NULL,
    name text NOT NULL
);


ALTER TABLE public.items OWNER TO tunneltime_user;

--
-- Name: items_id_seq; Type: SEQUENCE; Schema: public; Owner: tunneltime_user
--

CREATE SEQUENCE public.items_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.items_id_seq OWNER TO tunneltime_user;

--
-- Name: items_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: tunneltime_user
--

ALTER SEQUENCE public.items_id_seq OWNED BY public.items.id;


--
-- Name: mines; Type: TABLE; Schema: public; Owner: tunneltime_user
--

CREATE TABLE public.mines (
    id integer NOT NULL,
    town_id integer NOT NULL,
    total_stone integer NOT NULL,
    stone_density integer NOT NULL,
    CONSTRAINT mines_stone_density_check CHECK ((stone_density > 0)),
    CONSTRAINT mines_total_stone_check CHECK ((total_stone >= 0))
);


ALTER TABLE public.mines OWNER TO tunneltime_user;

--
-- Name: mines_id_seq; Type: SEQUENCE; Schema: public; Owner: tunneltime_user
--

CREATE SEQUENCE public.mines_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.mines_id_seq OWNER TO tunneltime_user;

--
-- Name: mines_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: tunneltime_user
--

ALTER SEQUENCE public.mines_id_seq OWNED BY public.mines.id;


--
-- Name: storage_building_levels; Type: TABLE; Schema: public; Owner: tunneltime_user
--

CREATE TABLE public.storage_building_levels (
    level integer NOT NULL,
    max_stone_count integer NOT NULL,
    CONSTRAINT storage_building_levels_max_stone_count_check CHECK ((max_stone_count > 0))
);


ALTER TABLE public.storage_building_levels OWNER TO tunneltime_user;

--
-- Name: storage_buildings; Type: TABLE; Schema: public; Owner: tunneltime_user
--

CREATE TABLE public.storage_buildings (
    id integer NOT NULL,
    town_id integer NOT NULL,
    level integer NOT NULL,
    current_stone_count integer NOT NULL,
    CONSTRAINT storage_buildings_current_stone_count_check CHECK ((current_stone_count >= 0))
);


ALTER TABLE public.storage_buildings OWNER TO tunneltime_user;

--
-- Name: storage_buildings_id_seq; Type: SEQUENCE; Schema: public; Owner: tunneltime_user
--

CREATE SEQUENCE public.storage_buildings_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.storage_buildings_id_seq OWNER TO tunneltime_user;

--
-- Name: storage_buildings_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: tunneltime_user
--

ALTER SEQUENCE public.storage_buildings_id_seq OWNED BY public.storage_buildings.id;


--
-- Name: store_front_buying_items; Type: TABLE; Schema: public; Owner: tunneltime_user
--

CREATE TABLE public.store_front_buying_items (
    store_front_id integer NOT NULL,
    item_id integer NOT NULL,
    gold integer NOT NULL
);


ALTER TABLE public.store_front_buying_items OWNER TO tunneltime_user;

--
-- Name: store_front_selling_items; Type: TABLE; Schema: public; Owner: tunneltime_user
--

CREATE TABLE public.store_front_selling_items (
    store_front_id integer NOT NULL,
    item_id integer NOT NULL,
    gold integer NOT NULL
);


ALTER TABLE public.store_front_selling_items OWNER TO tunneltime_user;

--
-- Name: store_fronts; Type: TABLE; Schema: public; Owner: tunneltime_user
--

CREATE TABLE public.store_fronts (
    id integer NOT NULL,
    town_id integer NOT NULL
);


ALTER TABLE public.store_fronts OWNER TO tunneltime_user;

--
-- Name: store_fronts_id_seq; Type: SEQUENCE; Schema: public; Owner: tunneltime_user
--

CREATE SEQUENCE public.store_fronts_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.store_fronts_id_seq OWNER TO tunneltime_user;

--
-- Name: store_fronts_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: tunneltime_user
--

ALTER SEQUENCE public.store_fronts_id_seq OWNED BY public.store_fronts.id;


--
-- Name: towns; Type: TABLE; Schema: public; Owner: tunneltime_user
--

CREATE TABLE public.towns (
    id integer NOT NULL,
    user_id integer NOT NULL,
    gold integer NOT NULL,
    CONSTRAINT towns_gold_gt_zero CHECK ((gold >= 0))
);


ALTER TABLE public.towns OWNER TO tunneltime_user;

--
-- Name: towns_id_seq; Type: SEQUENCE; Schema: public; Owner: tunneltime_user
--

CREATE SEQUENCE public.towns_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.towns_id_seq OWNER TO tunneltime_user;

--
-- Name: towns_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: tunneltime_user
--

ALTER SEQUENCE public.towns_id_seq OWNED BY public.towns.id;


--
-- Name: users; Type: TABLE; Schema: public; Owner: tunneltime_user
--

CREATE TABLE public.users (
    id integer NOT NULL,
    user_name text NOT NULL
);


ALTER TABLE public.users OWNER TO tunneltime_user;

--
-- Name: users_id_seq; Type: SEQUENCE; Schema: public; Owner: tunneltime_user
--

CREATE SEQUENCE public.users_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.users_id_seq OWNER TO tunneltime_user;

--
-- Name: users_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: tunneltime_user
--

ALTER SEQUENCE public.users_id_seq OWNED BY public.users.id;


--
-- Name: dwarves id; Type: DEFAULT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.dwarves ALTER COLUMN id SET DEFAULT nextval('public.dwarves_id_seq'::regclass);


--
-- Name: gem_shops id; Type: DEFAULT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.gem_shops ALTER COLUMN id SET DEFAULT nextval('public.gem_shops_id_seq'::regclass);


--
-- Name: gem_types id; Type: DEFAULT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.gem_types ALTER COLUMN id SET DEFAULT nextval('public.gem_types_id_seq'::regclass);


--
-- Name: gems id; Type: DEFAULT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.gems ALTER COLUMN id SET DEFAULT nextval('public.gems_id_seq'::regclass);


--
-- Name: items id; Type: DEFAULT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.items ALTER COLUMN id SET DEFAULT nextval('public.items_id_seq'::regclass);


--
-- Name: mines id; Type: DEFAULT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.mines ALTER COLUMN id SET DEFAULT nextval('public.mines_id_seq'::regclass);


--
-- Name: storage_buildings id; Type: DEFAULT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.storage_buildings ALTER COLUMN id SET DEFAULT nextval('public.storage_buildings_id_seq'::regclass);


--
-- Name: store_fronts id; Type: DEFAULT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.store_fronts ALTER COLUMN id SET DEFAULT nextval('public.store_fronts_id_seq'::regclass);


--
-- Name: towns id; Type: DEFAULT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.towns ALTER COLUMN id SET DEFAULT nextval('public.towns_id_seq'::regclass);


--
-- Name: users id; Type: DEFAULT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.users ALTER COLUMN id SET DEFAULT nextval('public.users_id_seq'::regclass);


--
-- Name: dwarf_mine_trips dwarf_mine_trips_dwarf_id_key; Type: CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.dwarf_mine_trips
    ADD CONSTRAINT dwarf_mine_trips_dwarf_id_key UNIQUE (dwarf_id);


--
-- Name: dwarves dwarves_name_unique; Type: CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.dwarves
    ADD CONSTRAINT dwarves_name_unique UNIQUE (town_id, name);


--
-- Name: dwarves dwarves_pkey; Type: CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.dwarves
    ADD CONSTRAINT dwarves_pkey PRIMARY KEY (id);


--
-- Name: gem_shops gem_shops_pkey; Type: CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.gem_shops
    ADD CONSTRAINT gem_shops_pkey PRIMARY KEY (id);


--
-- Name: gem_types gem_types_name_key; Type: CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.gem_types
    ADD CONSTRAINT gem_types_name_key UNIQUE (name);


--
-- Name: gem_types gem_types_pkey; Type: CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.gem_types
    ADD CONSTRAINT gem_types_pkey PRIMARY KEY (id);


--
-- Name: gems gems_pkey; Type: CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.gems
    ADD CONSTRAINT gems_pkey PRIMARY KEY (id);


--
-- Name: items items_name_key; Type: CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.items
    ADD CONSTRAINT items_name_key UNIQUE (name);


--
-- Name: items items_pkey; Type: CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.items
    ADD CONSTRAINT items_pkey PRIMARY KEY (id);


--
-- Name: mines mines_pkey; Type: CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.mines
    ADD CONSTRAINT mines_pkey PRIMARY KEY (id);


--
-- Name: storage_building_levels storage_building_levels_level_key; Type: CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.storage_building_levels
    ADD CONSTRAINT storage_building_levels_level_key UNIQUE (level);


--
-- Name: storage_buildings storage_buildings_pkey; Type: CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.storage_buildings
    ADD CONSTRAINT storage_buildings_pkey PRIMARY KEY (id);


--
-- Name: store_front_buying_items store_front_buying_items_store_front_id_item_id_key; Type: CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.store_front_buying_items
    ADD CONSTRAINT store_front_buying_items_store_front_id_item_id_key UNIQUE (store_front_id, item_id);


--
-- Name: store_front_selling_items store_front_selling_items_store_front_id_item_id_key; Type: CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.store_front_selling_items
    ADD CONSTRAINT store_front_selling_items_store_front_id_item_id_key UNIQUE (store_front_id, item_id);


--
-- Name: store_fronts store_fronts_pkey; Type: CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.store_fronts
    ADD CONSTRAINT store_fronts_pkey PRIMARY KEY (id);


--
-- Name: store_fronts store_fronts_town_id_key; Type: CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.store_fronts
    ADD CONSTRAINT store_fronts_town_id_key UNIQUE (town_id);


--
-- Name: towns towns_pkey; Type: CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.towns
    ADD CONSTRAINT towns_pkey PRIMARY KEY (id);


--
-- Name: towns towns_user_id_key; Type: CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.towns
    ADD CONSTRAINT towns_user_id_key UNIQUE (user_id);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: users users_user_name_key; Type: CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_user_name_key UNIQUE (user_name);


--
-- Name: dwarf_mine_trips dwarf_mine_trips_dwarf_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.dwarf_mine_trips
    ADD CONSTRAINT dwarf_mine_trips_dwarf_id_fkey FOREIGN KEY (dwarf_id) REFERENCES public.dwarves(id);


--
-- Name: dwarf_mine_trips dwarf_mine_trips_mine_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.dwarf_mine_trips
    ADD CONSTRAINT dwarf_mine_trips_mine_id_fkey FOREIGN KEY (mine_id) REFERENCES public.mines(id);


--
-- Name: gem_shop_gems gem_shop_gems_gem_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.gem_shop_gems
    ADD CONSTRAINT gem_shop_gems_gem_id_fkey FOREIGN KEY (gem_id) REFERENCES public.gems(id);


--
-- Name: gem_shop_gems gem_shop_gems_gem_shop_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.gem_shop_gems
    ADD CONSTRAINT gem_shop_gems_gem_shop_id_fkey FOREIGN KEY (gem_shop_id) REFERENCES public.gem_shops(id);


--
-- Name: gem_shops gem_shops_town_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.gem_shops
    ADD CONSTRAINT gem_shops_town_id_fkey FOREIGN KEY (town_id) REFERENCES public.towns(id);


--
-- Name: gems gems_gem_type_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.gems
    ADD CONSTRAINT gems_gem_type_id_fkey FOREIGN KEY (gem_type_id) REFERENCES public.gem_types(id);


--
-- Name: mines mines_town_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.mines
    ADD CONSTRAINT mines_town_id_fkey FOREIGN KEY (town_id) REFERENCES public.towns(id);


--
-- Name: storage_buildings storage_buildings_level_fkey; Type: FK CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.storage_buildings
    ADD CONSTRAINT storage_buildings_level_fkey FOREIGN KEY (level) REFERENCES public.storage_building_levels(level);


--
-- Name: storage_buildings storage_buildings_town_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.storage_buildings
    ADD CONSTRAINT storage_buildings_town_id_fkey FOREIGN KEY (town_id) REFERENCES public.towns(id);


--
-- Name: store_front_buying_items store_front_buying_items_item_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.store_front_buying_items
    ADD CONSTRAINT store_front_buying_items_item_id_fkey FOREIGN KEY (item_id) REFERENCES public.items(id);


--
-- Name: store_front_buying_items store_front_buying_items_store_front_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.store_front_buying_items
    ADD CONSTRAINT store_front_buying_items_store_front_id_fkey FOREIGN KEY (store_front_id) REFERENCES public.store_fronts(id);


--
-- Name: store_front_selling_items store_front_selling_items_item_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.store_front_selling_items
    ADD CONSTRAINT store_front_selling_items_item_id_fkey FOREIGN KEY (item_id) REFERENCES public.items(id);


--
-- Name: store_front_selling_items store_front_selling_items_store_front_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.store_front_selling_items
    ADD CONSTRAINT store_front_selling_items_store_front_id_fkey FOREIGN KEY (store_front_id) REFERENCES public.store_fronts(id);


--
-- Name: store_fronts store_fronts_town_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.store_fronts
    ADD CONSTRAINT store_fronts_town_id_fkey FOREIGN KEY (town_id) REFERENCES public.towns(id);


--
-- Name: towns towns_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.towns
    ADD CONSTRAINT towns_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id);


--
-- PostgreSQL database dump complete
--

