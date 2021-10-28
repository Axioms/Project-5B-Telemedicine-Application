# Project-5B-Telemedicine-Application

## How to run the Backend

### Setup Rust
> rustup default nightly

##### Note
if you want to use the rust language server run install the latest version of nightly that supports rls See https://rust-lang.github.io/rustup-components-history/
> rustup default nightly-2021-08-17

### Install diesel cli
> cargo install diesel_cli --no-default-features --features sqlite-bundled

### Create database
> diesel setup
Do this after you have configured the .env file

### Migrate the database
> diesel migration run

### Undo a migration
> diesel migration redo

### Build bin
> cargo build

### Run server
> cargo run

## Project Goal
This project is a web portal that can store messages/chat, reports, requests for appts, and host video recordings. We are possibly looking into making the site HIPPA compliant.
