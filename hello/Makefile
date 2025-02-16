rust-version:
	@echo "Rust command utility versions:"
	rustc --version 			# rust compiler
	cargo --version				# rust package manager
	rustfmt --version			# rust code formatter
	rustup --version			# rust tool chain manager
	clippy-driver --version 	#rust linter

install:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run

release:
	cargo build --release
all: format lint test run
