
test:
	cargo install --path .
	lambda_mountain --debug -o a.out tests/lm/tail.lm
	./a.out
#	lambda_mountain -o bootstrap BOOTSTRAP/cli.lm
#	./bootstrap --parse PRODUCTION/cli.lm
