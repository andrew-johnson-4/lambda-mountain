
dev: install-production
	cd $${HOME}; lm --c /home/andrew/workspace/lambda-mountain/tests/regress/lsts-frontend.lsts
	cc tmp.c
	./a.out
	echo $?

build-docs:
	lm --blob -o docs/index.html docs/index.html.lm
	#doby doc -o docs/default.html.html.lm PLATFORM/BLOB/LIB/default.html
	#doby doc -o docs/default.lm.html.lm PLATFORM/C/LIB/default.lm
	#doby doc -o docs/lm.lm.html.lm SRC/index-index.lm
	#lm --blob -o docs/default.html.html docs/default.html.html.lm
	#lm --blob -o docs/default.lm.html docs/default.lm.html.lm
	#lm --blob -o docs/lm.lm.html docs/lm.lm.html.lm

build: compile-production
	time ./production --c -o deploy.c SRC/index-index.lm
	cc deploy.c -o deploy
	time ./deploy --c -o deploy2.c SRC/index-index.lm
	diff deploy.c deploy2.c
	mv deploy.c BOOTSTRAP/cli.c
	rm -f deploy.c deploy2.c
	cargo test regression_tests

deploy: build build-docs

compile-bootstrap:
	rm -f bootstrap
	cc -o bootstrap BOOTSTRAP/cli.c

compile-production: compile-bootstrap
	rm -f production
	./bootstrap --c -o production.c SRC/index-index.lm
	cc -o production production.c
	rm -f production.c

install-production: compile-production
	mv production $${HOME}/bin/lm

install-bootstrap: compile-bootstrap
	mv bootstrap $${HOME}/bin/lm

install:
	cc -o lm BOOTSTRAP/cli.c
	mv lm $${HOME}/bin/lm
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
