# Disk9k1

A pomf.se and Google Drive clone.

# How to develop
## Server
Install a nightly Rust toolchain with `rustup`.

Install and start a PostgreSQL server and create the base `disk9k1`.
Create an ".env" file and populate it with environment variables:
```bash
DATABASE_URL=postgres://me@localhost:5432/disk9k1
ADMIN_USERNAME=whateveryouwant
ADMIN_EMAIL=randomEmail
ADMIN_PASSWORD=ditto
ROCKET_SECRET_KEY=look_it_up_in_Rocket_docs
```
Run Disk9k1 with `cargo run`. Disk9k1 will run the necessary database setup on its own.
## Frontend
Install npm, cd into `js` and run:
```bash
$ npm build
```
This will start a webpack watchdog that will watch for any changes and recompile the JavaScript in `js/src` to `js/dist/bundle.js`, which the server will serve under `/js/`.

Go back to the project root and start the server.
## Either
Alternatively, if you're on Linux and have [`just`](https://github.com/casey/just), run:
```bash
$ just develop
```
which takes care of all of the above for you - it starts the PostgreSQL SystemD service if it's not on yet, builds the server, runs the `app.js` build script with `npm` and finally starts the server.