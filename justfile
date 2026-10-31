# List all targets
default:
    @just --list --unsorted

# Format all code
[group('formatting')]
fmt:
    cargo fmt

# Check code formatting without changing it
[group('formatting')]
fmt-check:
    cargo fmt --all -- --check

# Run clippy to check for lints
[group('linting')]
check:
    cargo clippy --workspace --all-targets --all-features

# Build everything in the workspace
[group('building')]
build:
    cargo build --workspace --all-targets --all-features --release

# Build demo_01
[group('building')]
build-demo-01:
    cargo build -p demo_01 --release

# Run demo_01
[group('play')]
run-demo-01: build-demo-01
    cargo run -p demo_01 --release

# Clean the project
[group('other')]
clean:
    cargo clean