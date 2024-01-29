
test:
	cargo test bootsuite

forward:
	cargo install --path .
	lm --debug -o bootstrap BOOTSTRAP/cli.lm
	./bootstrap -o bootstrap.s BOOTSTRAP/cli.lm
	as -o bootstrap.o bootstrap.s
	ld -o bbootstrap bootstrap.o
