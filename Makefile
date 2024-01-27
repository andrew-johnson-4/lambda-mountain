
test:
	lambda_mountain -o bootstrap BOOTSTRAP/cli.lm
	./bootstrap --tokenize PRODUCTION/cli.lm
