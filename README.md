# Purpose

This will be an idle game to start with that
will be centered around dwarves digging for
resources and using those resources to improve
their tools and general capacity to dig more
resources.

# Installation

Install `rust` and `cargo` from your package
manager or from rustup:

https://github.com/rust-lang-nursery/rustup.rs/#other-installation-methods

I don't dig the curl into bash, use your
discretion if you do follow that path.

# Setup

To get the database running you can simply
snapshot with the snapshot script

> cd src/tunneltimeserver/db/

> ./snapshot.sh

If you already have data

> cd src/tunneltimeserver/db/

> ./drop.sh

> ./snapshot.sh

# Running

From the root of this project, you can run the
server and the client as follows:

> cargo run --manifest-path src/tunneltimeserver/Cargo.toml

> cargo run --manifest-path src/tunneltimeclient/Cargo.toml

# Data Management

In src/tunneltimeserver/db/ you can run the
following to manage adding new data and make
schema changes:

Clear your existing data:

> ./drop.sh

Restore from the snapshotted data:

> ./restore.sh

Snapshot your current database:

> ./snapshot.sh

Make schema change: create new file in migrations/

> psql -Utunneltime_user tunneltime -f migrations/YOUR_FILE.sql

> ./snapshot.sh

Add more data: create new file in boostrap/

> psql -Utunneltime_user tunneltime -f bootstrap/YOUR_FILE.sql

> ./snapshot.sh
