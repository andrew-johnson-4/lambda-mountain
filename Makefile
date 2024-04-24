
start: prod strict
	./strict --preprocess STDLIB/default-instruction-set.lm

nostd: prod strict
	./production --parse test.lm
	./strict --parse test.lm
	as -o tmp.o tmp.s
	ld -o tmp tmp.o
	./tmp STRICT/cli.lm && echo $?

strict: prod
	./production --nostd -o strict.s STRICT/cli.lm
	as -o strict.o strict.s
	ld -o strict   strict.o

tokenize: prod strict
	./production --tokenize STRICT/cli.lm > production-tokenize.txt
	./strict --tokenize STRICT/cli.lm > strict-tokenize.txt
	diff production-tokenize.txt strict-tokenize.txt > diff.txt
	cat diff.txt

parse: prod strict
	./production --parse --nomacro STRICT/cli.lm > production-parse.txt
	./strict --parse STRICT/cli.lm > strict-parse.txt
	diff production-parse.txt strict-parse.txt > diff.txt
	cat diff.txt

preprocess: prod strict
	./production --parse STDLIB/default-instruction-set.lm > production-preprocess.txt
	./strict --preprocess STDLIB/default-instruction-set.lm > strict-preprocess.txt
	diff production-preprocess.txt strict-preprocess.txt > diff.txt
	cat diff.txt

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
