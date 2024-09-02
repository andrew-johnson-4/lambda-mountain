
build-docs:
	lm --blob -o docs/index.html docs/index.html.lm

develop:
	lmv tests/regress/hello_world.s.v
	coqc tmp.v
	coqchk tmp.vo

deploy: compile-production
	time ./production -o deploy.s SRC/cli.lm
	as deploy.s -o deploy.o
	ld deploy.o -o deploy
	time ./deploy -o deploy2.s SRC/cli.lm
	diff deploy.s deploy2.s
	mv deploy.s BOOTSTRAP/cli.s
	cargo test regression_tests

compile-dev: compile-bootstrap
	rm -f dev dev.o dev.s
	./bootstrap -o dev.s DEV/cli.lm
	as -o dev.o dev.s
	ld -o dev   dev.o

compile-production: compile-bootstrap
	rm -f production production.o production.s
	./bootstrap -o production.s SRC/cli.lm
	as -o production.o production.s
	ld -o production   production.o
	cp production re-production

compile-bootstrap:
	rm -f bootstrap bootstrap.o
	as -o bootstrap.o BOOTSTRAP/cli.s
	ld -o bootstrap   bootstrap.o

install:
	as -o lm_raw.o BOOTSTRAP/cli.s
	ld -o lm lm_raw.o
	mv lm /usr/local/bin/
	rm lm_raw.o
	lm LMV/cli.lm -o lmv.s
	as -o lmv.o lmv.s
	ld -o lmv   lmv.o
	mv lmv /usr/local/bin
	rm lmv.s lmv.o

validate:
	coqc LIB/default_validator.v
	coqchk LIB/default_validator.vo

disassemble:
	objdump -S hello_world
