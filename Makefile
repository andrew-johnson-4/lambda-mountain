
test:
	cargo install --path .
	lambda_mountain --debug -o bootstrap BOOTSTRAP/cli.lm
	./bootstrap -o bootstrap.s PRODUCTION/cli.lm
#	./bootstrap -o bootstrap.s BOOTSTRAP/cli.lm


count:
	cargo install --path .
	lambda_mountain --debug -o bootstrap BOOTSTRAP/cli.lm
	./bootstrap --parse --stat PRODUCTION/cli.lm > parse.log
