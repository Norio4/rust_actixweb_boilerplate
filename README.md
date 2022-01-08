# rust_actixweb_boilerplate

The Rust Actix_Web Boilerplate is a boilerplate for developers who want to get started with Actix_web's web applications faster.


## Dependency

* cargo
* rustup
* libpq-dev


## Setup

* start postgrsql server
* rename .env.sample to .env
* change value of DATABASE_URL in .env
* run below commands

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
. "$HOME/.cargo/env"
rustup component add rls rust-analysis rust-src
cargo install cargo-edit
sudo apt install libpq-dev -y # Ubuntu
cargo install diesel_cli --no-default-features --features postgres
diesel setup
cargo run
```

## Recommend linker
  * [Mold](https://github.com/rui314/mold)
    * install
      ```
        sudo apt-get update -y; sudo apt-get install -y build-essential git clang cmake libstdc++-10-dev libssl-dev libxxhash-dev zlib1g-dev
        cd /tmp/
        git clone https://github.com/rui314/mold.git
        cd mold
        git checkout v1.0.1
        make -j$(nproc)
        sudo make install
      ```
    * update config.toml
      ```
        [target.x86_64-unknown-linux-gnu]
        linker = "clang"
        rustflags = ["-C", "link-arg=-fuse-ld=/path/to/mold"]
      ```

## Develop

```

$ ./action.sh

```

## Troubleshooting

###  failed to run custom build command for `openssl-sys v0.9.xx`

on Ubuntu

```
    apt-get update -y
    apt-get install -y libssl-dev
    apt-get install -y pkg-config
```


## In progress

- [ ] Session store to Redis
- [ ] OIDC login
- [ ] CSRF prevention
- [ ] Background Job
- [ ] Scaffold tool ( like rails generate )
