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
-- Data for Name: dwarves; Type: TABLE DATA; Schema: public; Owner: tunneltime_user
--

COPY public.dwarves (id, town_id, name) FROM stdin;
1	1	Idrigoth
2	1	Magni
\.


--
-- Name: dwarves_id_seq; Type: SEQUENCE SET; Schema: public; Owner: tunneltime_user
--

SELECT pg_catalog.setval('public.dwarves_id_seq', 2, true);


--
-- Data for Name: towns; Type: TABLE DATA; Schema: public; Owner: tunneltime_user
--

COPY public.towns (id, user_id, gold) FROM stdin;
1	1	100
\.


--
-- Data for Name: gem_shops; Type: TABLE DATA; Schema: public; Owner: tunneltime_user
--

COPY public.gem_shops (id, town_id) FROM stdin;
1	1
\.


--
-- Data for Name: gem_types; Type: TABLE DATA; Schema: public; Owner: tunneltime_user
--

COPY public.gem_types (id, name) FROM stdin;
1	ruby
2	sapphire
3	emerald
\.


--
-- Data for Name: gems; Type: TABLE DATA; Schema: public; Owner: tunneltime_user
--

COPY public.gems (id, gem_type_id, size) FROM stdin;
1	1	10
2	2	10
3	3	10
\.


--
-- Data for Name: gem_shop_gems; Type: TABLE DATA; Schema: public; Owner: tunneltime_user
--

COPY public.gem_shop_gems (gem_shop_id, gem_id) FROM stdin;
1	1
1	2
1	3
\.


--
-- Name: gem_shops_id_seq; Type: SEQUENCE SET; Schema: public; Owner: tunneltime_user
--

SELECT pg_catalog.setval('public.gem_shops_id_seq', 1, true);


--
-- Name: gem_types_id_seq; Type: SEQUENCE SET; Schema: public; Owner: tunneltime_user
--

SELECT pg_catalog.setval('public.gem_types_id_seq', 3, true);


--
-- Name: gems_id_seq; Type: SEQUENCE SET; Schema: public; Owner: tunneltime_user
--

SELECT pg_catalog.setval('public.gems_id_seq', 3, true);


--
-- Name: towns_id_seq; Type: SEQUENCE SET; Schema: public; Owner: tunneltime_user
--

SELECT pg_catalog.setval('public.towns_id_seq', 1, true);


--
-- PostgreSQL database dump complete
--

