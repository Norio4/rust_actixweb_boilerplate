# rust_actixweb_boilerplate

The Rust Actix_Web Boilerplate is a boilerplate for developers who want to get started with Actix_web's web applications faster.


## Dependency

* cargo
* rustup
* libpq-dev

## Setup

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup component add rls rust-analysis rust-src
cargo install cargo-edit
sudo apt install libpq-dev -y
cargo install diesel_cli --no-default-features --features postgres
diesel setup
cargo run
```


## In progress

* Session store to Redis
* OIDC login
* CSRF prevention
* Background Job
* Scaffold tool ( like rails generate )
