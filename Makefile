
test:
	lambda_mountain -o bootstrap BOOTSTRAP/cli.lm
	./bootstrap -o tmp.s tests/lm/uuid1.lm
	as -o tmp.o tmp.s
	ld -o a.out tmp.o
	./a.out
	#./bootstrap tests/lm/user_function_sugar1.lm
	#./bootstrap --parse PRODUCTION/cli.lm
