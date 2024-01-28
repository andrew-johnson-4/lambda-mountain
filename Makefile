
test:
	cargo install --path .
	lambda_mountain -o bootstrap BOOTSTRAP/cli.lm
	./bootstrap --parse PRODUCTION/cli.lm
