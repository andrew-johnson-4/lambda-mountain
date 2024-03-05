
test1: prod
	./production -o tmp.s tests/lm/dump_i.lm
	as tmp.s -o tmp.o
	ld tmp.o -o tmp
	./tmp

nostd: prod
	./production --typecheck --nostd -o tmp.s STDLIB/default-instruction-set.lm STDLIB/default-primitives.lm tests/nostd/ints.lm

test: prod
	./production -o production1.s PRODUCTION/cli.lm
	as -o production1.o production1.s
	ld -o production1   production1.o
	./production1 -o production2.s PRODUCTION/cli.lm
	as -o production2.o production2.s
	ld -o production2   production2.o

build:
	as -o lm_raw.o BOOTSTRAP/cli.s
	ld -o lm lm_raw.o
	rm lm_raw.o

prod: bs
	./bootstrap -o production.s PRODUCTION/cli.lm
	as -o production.o production.s
	ld -o production   production.o

bs:
	as -o bootstrap.o BOOTSTRAP/cli.s
	ld -o bootstrap   bootstrap.o

boot:
	lm -o bootstrap.s BOOTSTRAP/cli.lm
	as -o bootstrap.o bootstrap.s
	ld -o bootstrap   bootstrap.o

fresh:
	lm -o BOOTSTRAP/cli.s BOOTSTRAP/cli.lm

install:
	as -o lm_raw.o BOOTSTRAP/cli.s
	ld -o lm lm_raw.o
	mv lm /usr/local/bin/
	rm lm_raw.o
