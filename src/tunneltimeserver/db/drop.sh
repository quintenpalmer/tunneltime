#!/bin/bash
set -e

psql -U postgres -c "DROP DATABASE tunneltime"
psql -U postgres -c "DROP ROLE tunneltime_user"
