#!/bin/bash
set -e

pg_dump -s tunneltime > schema.sql
pg_dump -a tunneltime > data.sql
