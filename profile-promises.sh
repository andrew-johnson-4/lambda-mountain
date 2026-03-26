#/usr/bin/bash

rm -rf promises.prof

for fp in `ls tests/promises/lm-tctx/*.lsts`;
do
	echo "Processing promise file $fp"
	rm -f tmp.c a.out
	echo "Processing promise file $fp" >> promises.prof
	lm --showallocgen $fp
	if [ -f tmp.c ]; then
		gcc -w -O2 -march=native -mtune=native tmp.c -o a.out
		if [ -f a.out ]; then
			./a.out >> promises.prof
		fi
	fi
done

