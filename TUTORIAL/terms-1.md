
### Getting Started

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
import std/default.lm;

print("hello world\n");
```

To compile this we first call the `lm` command which will output a C source file.
The we can compile the C source file with `cc` or your favorite C compiler.
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

Functions are also declared with the `let` keyword with a list of arguments after the function name.

```
let f(x: U64, y: U64): U64 = x + y;

print( f(3, 4) );
```

### Primitive Control Flow

LSTS has if statements.

```
if condition { true-case } else { false-case }
```

and while loops.

```
while condition { do-something }
```

and even for-each iteration.

```
for x in [1,2,3] { print(X); }
```

### Advanced Control Flow

The pinnacle of control-flow in LSTS is the `match` expression.
A match expression attempts to destructure a value into a list of cases.
Each case has a left-hand-side which defines the conditions under which that case should match,
and a right-hand-side which defines what should be returned when that case matches.

In the simplest case, a match expression just checks equality against a literal value.

```
match 24 {
   1 => print("It's One.");
   4 => print("It's Four.");
   x => print("It's \{x}.");
}
```

However, there is so much more to this expression because left-hand-sides can be complicated.

Take for example string destructuring.

```
match "abc" {

   "d".. rest => print("Starts with a D.");
   # check to see if a string starts with a prefix

   r/^[0-9]/.. rest => print("Starts with a DIGIT.");
   # check to see if a string starts with a regular expression
}
```

Lists have a similar syntax for destructuring prefixes.

```
match [3,4,5] {
   [3.. 4.. rest] => print("List starts with 3, 4.");
}
```

Structured data can also be destructured in a match expression.

```
match (Point2D { 3, 4 }) {
   Point2D { y:4 } => print("Y is Four.");
   Point2d { x-value=x, y-value=y } => print("X is \{x-value}, Y is \{y-value}.");
}
```

