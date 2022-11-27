Thank you for your interest in Haven! If you haven't yet, be sure to check the [readme](/README.md) for a general idea of what Haven does and what its goals are, and the [license](/LICENSE) to see how your contributions can be used.

## Introduction

The project is split into two parts, the backend and frontend. The backend is written in [Rust](https://www.rust-lang.org/) and the frontend in [Elm](https://elm-lang.org/). Both languages offer good compile-time guarantees, so even if you're not confident in your skills in one or both of the languages, their compilers will guide you along as you make changes.

Additionally, the types the frontend uses to communicate with the backend are automatically generated from the backend types, meaning that changes to said types will result in compilation errors in the frontend, so hack away!

## Backend

The backend is a typical web server that offers a REST API and uses JSON to communicate.

### Requirements
- [Rust](https://www.rust-lang.org/tools/install) (The rustfmt config requires nightly, ensure it's installed with `rustup install nightly`)
- [PostgreSQL](https://www.postgresql.org/download/)
- [Diesel CLI](https://crates.io/crates/diesel_cli) (`cargo install diesel_cli --no-default-features --features "postgres"`)

### Notable libraries used
- [Axum](https://github.com/tokio-rs/axum) for the server.
- [Diesel](https://diesel.rs/) for interfacing with the PostgreSQL database.
- [serde_json](https://crates.io/crates/serde_json) and [elm_rs](https://crates.io/crates/elm_rs) for facilitating communication between the backend and frontend.

### Formatting and linting
- `cargo +nightly fmt`
- `cargo clippy`

### Setting up the database
A user for the development database can be created with

```sql
CREATE USER haven WITH CREATEDB PASSWORD 'haven';
```

With the user set up, run `diesel setup`.

## Frontend

The frontend is a single-page application (SPA).

### Requirements
- [Elm](https://guide.elm-lang.org/install/elm.html)
- [elm-spa](https://www.elm-spa.dev/guide#installation)
- [elm-format](https://github.com/avh4/elm-format)
- [elm-review](https://github.com/jfmengels/elm-review)

### Notable libraries
- [elm-spa](https://www.elm-spa.dev/) as the framework.
- [Bulma](https://bulma.io/) for styling.

### Formatting and linting
- `elm-format --yes src`
- `elm-review`


## dev.sh
The repository root contains a script `dev.sh` that starts the backend and frontend and automatically re-generates the Elm API types when they are changed in the backend. Requires [Watchexec CLI](https://crates.io/crates/watchexec-cli).
