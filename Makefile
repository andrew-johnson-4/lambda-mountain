
develop: compile-strict
	./strict -o tmp.s tests/strict/printU64.lm
	as tmp.s -o tmp.o
	ld tmp.o -o tmp
	./tmp

compile-strict: compile-prod
	./production --nostd -o strict.s STRICT/cli.lm
	as -o strict.o strict.s
	ld -o strict   strict.o

compile-prod: compile-bootstrap
	./bootstrap -o production.s PRODUCTION/cli.lm
	as -o production.o production.s
	ld -o production   production.o

compile-bootstrap:
	as -o bootstrap.o BOOTSTRAP/cli.s
	ld -o bootstrap   bootstrap.o

install:
	as -o lm_raw.o BOOTSTRAP/cli.s
	ld -o lm lm_raw.o
	mv lm /usr/local/bin/
	rm lm_raw.o
