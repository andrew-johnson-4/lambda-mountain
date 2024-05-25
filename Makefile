
develop: compile-strict
	cp strict re-strict
	rm -f strict-loop strict-loop.o strict-loop.s
	./strict -o strict-loop.s SRC/cli.lm
	as strict-loop.s -o strict-loop.o
	ld strict-loop.o -o strict-loop
	cp strict-loop re-strict-loop
	./strict-loop -o strict-loop-2.s SRC/cli.lm

re:
	./re-strict -o tmp.s tests/btstrp/test23.lm
	as tmp.s -o tmp.o
	ld tmp.o -o tmp
	./tmp

compile-strict: compile-prod
	rm -f strict strict.o strict.s
	./production --nostd -o strict.s STRICT/cli.lm
	as -o strict.o strict.s
	ld -o strict   strict.o

compile-prod: compile-bootstrap
	rm -f production production.o production.s
	./bootstrap -o production.s PRODUCTION/cli.lm
	as -o production.o production.s
	ld -o production   production.o

compile-bootstrap:
	rm -f bootstrap bootstrap.o
	as -o bootstrap.o BOOTSTRAP/cli.s
	ld -o bootstrap   bootstrap.o

install:
	as -o lm_raw.o BOOTSTRAP/cli.s
	ld -o lm lm_raw.o
	mv lm /usr/local/bin/
	rm lm_raw.o
