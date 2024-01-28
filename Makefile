
test:
	lambda_mountain -o bootstrap BOOTSTRAP/cli.lm
	./bootstrap -o tmp.s tests/lm/user_function_unsugared.lm
	as -o tmp.o tmp.s
	ld -o a.out tmp.o
	./a.out
#	./bootstrap --parse PRODUCTION/cli.lm
