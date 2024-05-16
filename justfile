default: build

build:
    cargo build

build-release:
    cargo build --release

build-static:
    cargo build --target x86_64-unknown-linux-musl

build-release-static:
    cargo build --release --target x86_64-unknown-linux-musl

run *ARGS:
    cargo run -q -- {{ARGS}}

install:
    cargo install --path .
