
default:

fmt:
    cargo fmt

check:
    cargo clippy --workspace --all-targets --all-features

run:
    cargo run