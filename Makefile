.PHONY: test-unit check build test-spec test

test: test-unit build test-spec

test-unit: check
	cargo test --workspace --exclude specsafely

check:
	cargo fmt -- --check
	cargo clippy

build:
	cargo build --release

test-spec:
	cargo test --package specsafely