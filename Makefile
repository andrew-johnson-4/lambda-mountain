
test:
	cargo install --path .
	lambda_mountain --debug -o bootstrap BOOTSTRAP/cli.lm
	./bootstrap --parse tests/lm/123.lm
#	cargo test parseall
#	./bootstrap --parse --stat PRODUCTION/cli.lm

count:
	cargo install --path .
	lambda_mountain --debug -o bootstrap BOOTSTRAP/cli.lm
	./bootstrap --parse --stat PRODUCTION/cli.lm > parse.log
