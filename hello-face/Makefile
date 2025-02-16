.PHONY: rust-version install format lint test run release all

rust-version:
	@echo "Rust command utility versions:"
	rustc --version			# Rust compiler
	cargo --version			# Rust package manager
	rustfmt --version		# Rust code formatter
	rustup --version		# Rust toolchain manager
	clippy-driver --version	# Rust linter

install:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

format:
	cargo fmt --manifest-path ./Cargo.toml --quiet

lint:
	cargo clippy --manifest-path ./Cargo.toml --quiet

test:
	cargo test --manifest-path ./Cargo.toml --quiet

run:
	cargo run --manifest-path ./Cargo.toml

release:
	cargo build --manifest-path ./Cargo.toml --release

all: format lint test run