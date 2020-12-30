test:
    cargo test --all
    cargo test --all --no-default-features --features use_yare
    cargo test --all --no-default-features --features use_parameterized

fmt:
    cargo fmt --all
