
test:
	cargo install --path .
	lambda_mountain -o bootstrap BOOTSTRAP/cli.lm
	./bootstrap --parse tests/lm/123.lm

todo:
	cargo test parseall
