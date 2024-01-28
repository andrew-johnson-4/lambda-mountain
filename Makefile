
test:
	cargo install --path .
	lambda_mountain --parse tests/lm/uve.lm
	lambda_mountain --debug -o a.out tests/lm/uve.lm
	./a.out

todo:
	#lambda_mountain -o bootstrap BOOTSTRAP/cli.lm
	#./bootstrap test/lm/uve.lm
	#as -o tmp.o tmp.s
	#ld -o a.out tmp.o
	#./a.out
#	./bootstrap --parse PRODUCTION/cli.lm
