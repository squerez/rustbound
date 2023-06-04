check-linting:
	cargo clippy --all-targets --all-features -- -D warnings

fix-linting:
	cargo clippy --fix

run-tests:
	cargo test

run:
	cargo build && cargo run
