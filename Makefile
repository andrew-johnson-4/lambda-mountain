
develop: compile-strict
	rm -f tmp tmp.o tmp.s
	./strict -o tmp.s tests/btstrp/test3.lm
	as tmp.s -o tmp.o
	ld tmp.o -o tmp
	./tmp

re:
	rm -f tmp tmp.o tmp.s
	./strict -o tmp.s tests/strict/head-string.lm
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
