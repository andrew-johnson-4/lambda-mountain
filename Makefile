
test:
	cargo install --path .
	lambda_mountain --debug -o bootstrap BOOTSTRAP/cli.lm
	cargo test parseall

todo:
	./bootstrap --parse tests/lm/123.lm
