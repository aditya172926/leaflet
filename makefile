.PHONY: build release

fmt:
	cargo fmt --all

build:
	cargo fmt --all
	cargo check --all
	cargo build --all

release:
	cargo fmt --all
	cargo check --all
	cargo build --all --release