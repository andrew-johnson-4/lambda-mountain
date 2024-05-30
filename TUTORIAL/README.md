This tutorial is very short and assumes familiarity with programming in general.

### Import a Library

Libraries are files relative to the current working directory.

```
import LIB/default-instruction-set.lm;
```

### Tokenization of Syntax

A variable starts with a lower case letter usually.
Anything that isn't a variable will become a Literal.
Variable names can be turned into Literals by prefixed them with a single quote/tick mark: `'abc`.

### Declare a Function

Global Terms such as functions must be given explicit types. They are bound with the walrus `:=` operator.

```
x := (: 123 U64);

f := λ(: x X). (: x X);
```

### What is a Fragment?

Fragments are bound in a similar fashion to Global Terms.
However only the external interface of a fragment is typed.
The internals of a fragment define a rendering template that yields code fragments.

```
fragment f := λ(: x X). (: (
   (.program( \t mov \s x , \s %rax \n ))
) F);
```
