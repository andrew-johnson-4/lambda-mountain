
test:
	cargo install --path .
	lambda_mountain --debug -o bootstrap BOOTSTRAP/cli.lm

todo:
#	cargo test parseall
#	./bootstrap --parse PRODUCTION/cli.lm
