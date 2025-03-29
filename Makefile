# Makefile

dev:
	cargo build
	cargo install --path .

run:
	cargo run

install:
	cargo install --path .

clean:
	cargo clean
