# Rust API

## Steps to run the repo:

### Install Postgress
-  https://www.postgresql.org/download/

### Install Diesel CLI
- `cargo install diesel_cli --no-default-features --features postgres`

### Setup Diesel for your project
- run command in rust project directory terminal
- `echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env`
- `diesel setup`
