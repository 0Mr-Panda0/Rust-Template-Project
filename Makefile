install:
	# Install if needed
	#@echo "Updating rust toolchain"
	#rustup update stable
	#rustup default stable

rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	@echo "Formatting project with cargo"
	cd hello_world && cargo fmt --all

lint:
	@echo "Linting project with cargo"
	cd hello_world  && cargo clippy

test:
	@echo "Testing project with cargo"
	cd hello_world  && cargo test


all: format lint test