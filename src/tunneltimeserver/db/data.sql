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
7	1	Prudent
\.


--
-- Name: dwarves_id_seq; Type: SEQUENCE SET; Schema: public; Owner: tunneltime_user
--

SELECT pg_catalog.setval('public.dwarves_id_seq', 7, true);


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: tunneltime_user
--

COPY public.users (id, user_name) FROM stdin;
1	postprompt
2	other user
3	qp
4	qptest
5	qptestagain
\.


--
-- Data for Name: towns; Type: TABLE DATA; Schema: public; Owner: tunneltime_user
--

COPY public.towns (id, user_id, gold) FROM stdin;
2	2	100
3	3	100
1	1	80
4	4	100
5	5	100
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
-- Data for Name: mines; Type: TABLE DATA; Schema: public; Owner: tunneltime_user
--

COPY public.mines (id, town_id, total_stone, stone_density) FROM stdin;
1	1	1000	5
2	2	1000	5
3	3	1000	5
4	4	1000	5
5	5	1000	5
\.


--
-- Name: mines_id_seq; Type: SEQUENCE SET; Schema: public; Owner: tunneltime_user
--

SELECT pg_catalog.setval('public.mines_id_seq', 5, true);


--
-- Data for Name: storage_building_levels; Type: TABLE DATA; Schema: public; Owner: tunneltime_user
--

COPY public.storage_building_levels (level, max_stone_count) FROM stdin;
1	100
2	200
3	400
4	800
5	1600
6	3200
7	6400
8	12800
9	25600
10	51200
\.


--
-- Data for Name: storage_buildings; Type: TABLE DATA; Schema: public; Owner: tunneltime_user
--

COPY public.storage_buildings (id, town_id, level, current_stone_count) FROM stdin;
1	4	1	0
2	1	1	0
3	2	1	0
4	3	1	0
5	5	1	0
\.


--
-- Name: storage_buildings_id_seq; Type: SEQUENCE SET; Schema: public; Owner: tunneltime_user
--

SELECT pg_catalog.setval('public.storage_buildings_id_seq', 5, true);


--
-- Name: towns_id_seq; Type: SEQUENCE SET; Schema: public; Owner: tunneltime_user
--

SELECT pg_catalog.setval('public.towns_id_seq', 5, true);


--
-- Name: users_id_seq; Type: SEQUENCE SET; Schema: public; Owner: tunneltime_user
--

SELECT pg_catalog.setval('public.users_id_seq', 5, true);


--
-- PostgreSQL database dump complete
--

