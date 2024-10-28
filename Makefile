
dev:
	lm --c tests/regress/seq-macro.lm
	cc tmp.c
	./a.out
	echo $?

profile: install-production
	lm --profile-invocations SRC/index-index.lm -o profile.s
	as profile.s -o profile.o
	ld profile.o -o profile
	./profile SRC/index-index.lm --typecheck | sort -n

build-docs:
	lm --blob -o docs/index.html docs/index.html.lm
	doby doc -o docs/default.html.html.lm LIB/default.html
	doby doc -o docs/default.lm.html.lm LIB/default.lm
	doby doc -o docs/lm.lm.html.lm SRC/index-index.lm
	lm --blob -o docs/default.html.html docs/default.html.html.lm
	lm --blob -o docs/default.lm.html docs/default.lm.html.lm
	lm --blob -o docs/lm.lm.html docs/lm.lm.html.lm
	lm --blob -o docs/test.html docs/test.html.lm

develop:
	lmv tests/regress/hello_world.s.v
	coqc tmp.v
	coqchk tmp.vo

build: compile-production
	time ./production --gnu -o deploy.s SRC/index-index.lm
	as deploy.s -o deploy.o
	ld deploy.o -o deploy
	time ./deploy --gnu -o deploy2.s SRC/index-index.lm
	diff deploy.s deploy2.s
	mv deploy.s BOOTSTRAP/cli.s
	cargo test regression_tests

deploy: build build-docs

compile-production: compile-bootstrap
	rm -f production production.o production.s
	./bootstrap --gnu -o production.s SRC/index-index.lm
	as -o production.o production.s
	ld -o production   production.o
	cp production re-production

install-production: compile-production
	mv production $${HOME}/bin/lm

install-bootstrap: compile-bootstrap
	mv bootstrap $${HOME}/bin/lm

compile-bootstrap:
	rm -f bootstrap bootstrap.o
	as -o bootstrap.o BOOTSTRAP/cli.s
	ld -o bootstrap   bootstrap.o

install:
	as -o lm_raw.o BOOTSTRAP/cli.s
	ld -o lm lm_raw.o
	mv lm $${HOME}/bin/lm
	rm lm_raw.o
	#lm LMV/cli.lm -o lmv.s
	#as -o lmv.o lmv.s
	#ld -o lmv   lmv.o
	#mv lmv /usr/local/bin
	#rm lmv.s lmv.o
	#lm DOBY/cli.lm -o doby.s
	#as -o doby.o doby.s
	#ld -o doby   doby.o
	#mv doby /usr/local/bin
	#rm doby.s doby.o

validate:
	coqc LIB/default_validator.v
	coqchk LIB/default_validator.vo

disassemble:
	objdump -S hello_world
