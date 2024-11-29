
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

### Print different kinds of values

Now that we are setup, we can explore all the different kinds of values that are available in LSTS.

Like numbers...

```
print(1 - 3 * -4.5 / 7e23);
```

Or strings...

```
print( "abcd"[2:] + "efg"[:-1] );
```

We have lists, sets, and maps.

```
print([ 1, 2, 2, 3 ]);
print({ 1, 2, 2, 3 });
print({ 1: 2, 2: 3 });
```

And regular expressions.

```
r/abc+/.matches("abcccc")
```

There are many other values available in LSTS, but we'll start with these.

### Variables and Functions

Variables store values for use later and are declared with the `let` keyword.

```
let x = 5;
let y = "abc";
let z = [6,7,8];
print( x + y.length + z.length );
```

Functions are also declared with the `let` keyword with a list of arguments after the name.

```
let f(x: U64, y: U64): U64 = x + y;

print( f(3, 4) );
```
