
develop: compile-strict
	cp strict re-strict
	rm -f strict-loop strict-loop.o strict-loop.s
	./strict -o strict-loop.s STRICT/cli.lm
	as strict-loop.s -o strict-loop.o
	ld strict-loop.o -o strict-loop
	./strict-loop --tokenize STRICT/cli.lm

develop-2: compile-strict
	cp strict re-strict
	rm -f tmp tmp.o tmp.s
	./strict -o tmp.s tests/btstrp/test17.lm
	as tmp.s -o tmp.o
	ld tmp.o -o tmp
	./tmp

re:
	cp strict re-strict
	rm -f tmp tmp.o tmp.s
	./strict -o tmp.s tests/btstrp/test20.lm
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
