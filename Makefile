
test:
	cargo install --path .
	cargo test bootsuite

count:
	cargo install --path .
	lambda_mountain --debug -o bootstrap BOOTSTRAP/cli.lm
	./bootstrap --parse --stat PRODUCTION/cli.lm > parse.log
