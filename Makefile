CC = clang
CFLAGS = -w -O2 -march=native -mtune=native
LSTSFLAGS = MALLOC_CHECK_=3

# WARNING: You may need to increase ulimit
# the compiler stack frames are currently fairly fat and inefficient
# recursion is used fairly heavily
# recommendation: ulimit -s unlimited

dev: install-production
	lm tests/promises/match/constant-strings.lsts
	gcc tmp.c
	./a.out
	#time lm --showalloc SRC/unit-type-core.lsts > out.txt
	#time lm --showalloc SRC/unit-tctx-core.lsts > out.txt
	#time lm --showalloc SRC/unit-prop-core.lsts > out.txt
	#time lm --showalloc SRC/unit-ascript-core.lsts > out.txt
	#time lm --showalloc SRC/index.lsts > out.txt
	#time lm --showalloc SRC/index.lsts > out.txt

build: compile-production
	time env $(LSTSFLAGS) ./production --v23 -o deploy1.c SRC/dev-index.lsts > out1.txt
	$(CC) $(CFLAGS) deploy1.c -o deploy1
	time env $(LSTSFLAGS) ./deploy1 --v23 -o deploy2.c SRC/dev-index.lsts > out2.txt
	diff deploy1.c deploy2.c
	mv deploy1.c BOOTSTRAP/cli.c
	rm -f deploy1 deploy1.c deploy2.c
	cargo test regression_tests

deploy: build smoke-test
deploy-lite: build smoke-test-lite

gprofng: install-production
	gprofng collect app lm SRC/index.lsts

gprofng-view:
	gprofng display text -functions test.1.er > gprofng.view
	nano gprofng.view

valgrind: install-production
	valgrind --tool=callgrind lm SRC/index.lsts

valgrind-view:
	callgrind_annotate callgrind.out.18778

gprof:
	$(CC) $(CFLAGS) -pg -o bootstrap.exe BOOTSTRAP/cli.c
	$(LSTSFLAGS) ./bootstrap.exe SRC/index.lsts

gprof-view-count:
	gprof bootstrap.exe gmon.out | less

gprof-view-call-graph:
	gprof -q bootstrap.exe gmon.out

profile: install-bootstrap
	perf record lm --v2 SRC/index.lsts
	./report.sh

compile-bootstrap:
	rm -f bootstrap.exe
	$(CC) $(CFLAGS) -o bootstrap.exe BOOTSTRAP/cli.c

compile-production: compile-bootstrap
	rm -f production
	$(LSTSFLAGS) ./bootstrap.exe --v23 -o production.c SRC/index.lsts
	$(CC) $(CFLAGS) -o production production.c
	rm -f production.c

install-production: compile-production
ifeq ($(shell test -w /usr/local/bin; echo $$?), 0)
	cp production /usr/local/bin/lm-production
	mv production /usr/local/bin/lm
else
	mkdir -p $${HOME}/.local/bin
	cp production $${HOME}/.local/bin/lm-production
	mv production $${HOME}/.local/bin/lm
endif

install-bootstrap: compile-bootstrap
ifeq ($(shell test -w /usr/local/bin; echo $$?), 0)
	cp bootstrap.exe /usr/local/bin/lm-bootstrap
	mv bootstrap.exe /usr/local/bin/lm
else
	mkdir -p $${HOME}/.local/bin
	cp bootstrap.exe $${HOME}/.local/bin/lm-bootstrap
	mv bootstrap.exe $${HOME}/.local/bin/lm
endif

smoke-test-clang:
	clang $(CFLAGS) BOOTSTRAP/cli.c -o tmp
	rm tmp

smoke-test-gcc:
	gcc $(CFLAGS) BOOTSTRAP/cli.c -o tmp
	rm tmp

smoke-test-musl:
	musl-gcc $(CFLAGS) BOOTSTRAP/cli.c -o tmp
	rm tmp

smoke-test: smoke-test-clang smoke-test-gcc smoke-test-musl
smoke-test-lite: smoke-test-clang smoke-test-gcc

install:
	$(CC) $(CFLAGS) -o lm BOOTSTRAP/cli.c
ifeq ($(shell test -w /usr/local/bin; echo $$?), 0)
	mv lm /usr/local/bin/lm
else
	mkdir -p $${HOME}/.local/bin
	mv lm $${HOME}/.local/bin/lm
endif
	mkdir -p $${HOME}/.lm/
	cp -rf lib $${HOME}/.lm/
