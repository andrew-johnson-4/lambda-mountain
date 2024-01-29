
test:
	cargo test bootsuite

count:
	cargo install --path .
	lambda_mountain --debug -o bootstrap BOOTSTRAP/cli.lm
	./bootstrap -o lm.s BOOTSTRAP/cli.lm
	as -o lm.o lm.s
	ld -o lm lm.o
	./lm -o lm2.s BOOTSTRAP/cli.lm
	as -o lm2.o lm2.s
	ld -o lm2 lm2.o
