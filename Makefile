
p:
	lm -o production.s PRODUCTION/cli.lm
	as -o production.o production.s
	ld -o production   production.o
	./production --parse tests/lm/user_function_unsugared.lm

prod:
	cargo test prodsuite

test:
	cargo test testsuite

build:
	as -o lm_raw.o BOOTSTRAP/cli.s
	ld -o lm lm_raw.o
	rm lm_raw.o

install:
	as -o lm_raw.o BOOTSTRAP/cli.s
	ld -o lm lm_raw.o
	mv lm /usr/local/bin/
	rm lm_raw.o
