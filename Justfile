# use with https://github.com/casey/just

_default:
    @just --list

check:
    cargo clippy

build:
    cargo build --locked
    cargo fmt

build-release:
    cargo build --locked --release
    cargo fmt

fmt:
    cargo fmt