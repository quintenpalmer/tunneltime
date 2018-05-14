#!/bin/bash
set -e

if [ $(psql -d postgres -q -t -c "select count(*) from pg_database where datname = 'tunneltime'") == 1 ]; then
    echo "you already have a tunneltime database, delete it first to restore"
    exit
fi

psql -d postgres -c "CREATE DATABASE tunneltime"
if [ $(psql -d postgres -q -t -c "select count(*) from pg_roles where rolname = 'tunneltime_user'") == 0 ]; then
    echo "you already have a tunneltime_user user, delete it first to restore"
    psql -d postgres -c "CREATE ROLE tunneltime_user WITH CREATEDB LOGIN"
    psql -d postgres -c "GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA PUBLIC TO tunneltime_user"
fi

echo "restoring the schema"
psql -U tunneltime_user -d tunneltime -f schema.sql
echo "done restoring the schema"

echo "inserting seed data"
psql -U tunneltime_user -d tunneltime -f data.sql
echo "done inserting data"
exit
