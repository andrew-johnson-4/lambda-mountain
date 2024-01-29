
test:
	cargo install --path .
	lambda_mountain --debug -o bootstrap BOOTSTRAP/cli.lm
	./bootstrap -o bootstrap.s BOOTSTRAP/cli.lm
	as -o tmp.o tmp.s
	ld -o lm tmp.o

count:
	cargo install --path .
	lambda_mountain --debug -o bootstrap BOOTSTRAP/cli.lm
	./bootstrap --parse --stat PRODUCTION/cli.lm > parse.log
