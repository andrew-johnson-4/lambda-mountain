
test:
	cargo install --path .
	lambda_mountain --debug -o bootstrap BOOTSTRAP/cli.lm
	./bootstrap -o lm.s BOOTSTRAP/cli.lm
	as -o lm.o lm.s
	ld -o lm lm.o
	./lm -o lm2.s BOOTSTRAP/cli.lm

count:
	cargo test bootsuite
	cargo install --path .
	lambda_mountain --debug -o bootstrap BOOTSTRAP/cli.lm
	./bootstrap --parse --stat PRODUCTION/cli.lm > parse.log
