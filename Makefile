
test:
	cargo install --path .
	lambda_mountain -o bootstrap BOOTSTRAP/cli.lm
	./bootstrap tests/lm/uve.lm
	as -o tmp.o tmp.s
	ld -o a.out tmp.o
	./a.out

todo:
#	./bootstrap --parse PRODUCTION/cli.lm
