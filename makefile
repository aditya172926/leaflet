.PHONY: build release

build:
	cargo fmt --all
	cargo check --all
	cargo build --all

release:
	cargo fmt --all
	cargo check --all
	cargo build --all --release