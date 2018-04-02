test: install bdd

bdd:
	@make -C tests bdd

install:
	cargo install --force
