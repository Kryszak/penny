[default]
_list_available:
    @just --list --unsorted

# Run tests
[group('Build')]
test:
    cargo test

# Build project
[group('Build')]
build:
    cargo build

# Check for errors
[group('Lint')]
lint:
    cargo clippy -- -D warnings

# Autofix clippy errors
[group('Lint')]
lint-fix:
    cargo clippy --fix -- -D warnings

# Run project
[group('Dev')]
run:
    cargo run
