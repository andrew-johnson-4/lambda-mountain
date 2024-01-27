
test:
	lambda_mountain -o bootstrap BOOTSTRAP/cli.lm
	./bootstrap tests/lm/user_function_sugar1.lm
	#./bootstrap --parse PRODUCTION/cli.lm
