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
-- Name: towns; Type: TABLE; Schema: public; Owner: tunneltime_user
--

CREATE TABLE public.towns (
    id integer NOT NULL,
    user_id integer NOT NULL
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
-- Name: towns id; Type: DEFAULT; Schema: public; Owner: tunneltime_user
--

ALTER TABLE ONLY public.towns ALTER COLUMN id SET DEFAULT nextval('public.towns_id_seq'::regclass);


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
-- PostgreSQL database dump complete
--

