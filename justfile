_default: help

# Show possible commands
@help:
    just --list --unsorted

# Runs a development instance of the server
[group('development')]
run:
    cargo run
alias r := run

# Builds the project
[group('development')]
build:
    cargo build
alias b := build

# Runs the workspace's tests
[group('development')]
test:
    cargo test --all-features --all
alias t := test

# Runs clippy on the workspace
[group('development')]
clippy:
    cargo clippy --all-features
alias lint := clippy

# Formats the project
[group('development')]
fmt:
    cargo +nightly fmt
