
test:
	lambda_mountain -o a.out tests/lm/tail.lm
	./a.out
	cargo install --path .
	lambda_mountain -o bootstrap BOOTSTRAP/cli.lm
	./bootstrap --parse PRODUCTION/cli.lm
