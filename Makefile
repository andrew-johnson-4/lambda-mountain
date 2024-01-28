
test:
	cargo install --path .
	lambda_mountain --debug -o bootstrap BOOTSTRAP/cli.lm
	cargo test parseall
	./bootstrap --parse --stat PRODUCTION/cli.lm
