
LSTS is a typed macro assembler that by default compiles to C.
If you would like to follow along with this tutorial you can download and install it on any Posix operating system.

```
git clone https://github.com/andrew-johnson-4/lambda-mountain.git
cd lambda-mountain
make install
```

This will install an `lm` command that we will use to compile our source files.

### Import the standard library

To run our examples we will make an example file `example.lsts`.

```
import LIB/default.lm;

print("hello world\n");
```

To compile this we first call the `lm` command which will output a C source file.
The we can compile the C source file with `cc` or your favorite C compile.
This will give us an executable which we can finally run.

```
lm example.lsts -o example.c
cc example.c -o example
./example
```
