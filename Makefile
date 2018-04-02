test: clippy bdd

clippy:
	cargo +nightly clippy

bdd: install
	@make -C tests bdd

install:
	cargo install --force
