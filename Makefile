
test-dbg:
	as -o pp.o pp.s
	ld -o pp   pp.o
	./pp

test-prod: prod
	./production -o pp.s tests/lm/match13.lm
	as -o pp.o pp.s
	ld -o pp   pp.o
	./pp

test:
	cargo test testsuite

build:
	as -o lm_raw.o BOOTSTRAP/cli.s
	ld -o lm lm_raw.o
	rm lm_raw.o

prod:
	lm -o production.s PRODUCTION/cli.lm
	as -o production.o production.s
	ld -o production   production.o

fresh:
	lm -o BOOTSTRAP/cli.s BOOTSTRAP/cli.lm

install:
	as -o lm_raw.o BOOTSTRAP/cli.s
	ld -o lm lm_raw.o
	mv lm /usr/local/bin/
	rm lm_raw.o
