
test:
	cargo install --path .
	lambda_mountain -o bootstrap BOOTSTRAP/cli.lm
	./bootstrap test/lm/uve.lm
	as -o tmp.o tmp.s
	ld -o a.out tmp.o

todo:
	#./a.out
#	./bootstrap --parse PRODUCTION/cli.lm
