
test:
	cargo install --path .
	lambda_mountain -o bootstrap BOOTSTRAP/cli.lm
	cargo test parseall
	./bootstrap --parse --stat PRODUCTION/cli.lm
