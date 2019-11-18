# Disk9k1

A pomf.se and Google Drive clone.

# How to develop
## Server
Install a nightly Rust toolchain with `rustup`. Install `diesel-cli` with `cargo install diesel-cli`.
Install and start a PostgreSQL server and create the base `disk9k1`.
Create an ".env" file and populate it with environment variables:
```bash
DATABASE_URL=postgres://me@localhost:5432/disk9k1
ADMIN_USERNAME=whateveryouwant
ADMIN_PASSWORD=ditto
ADMIN_TOKEN=samegoesforthis
ROCKET_SECRET_KEY=look_it_up_in_Rocket_docs
```
Run the DB migrations with `diesel migration run`.
Run Disk9k1 with `cargo run`.
## Frontend
Install npm, cd into `js` and run:
```bash
$ npm start
```
Start the server.