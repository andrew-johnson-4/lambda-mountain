This tutorial is very short and assumes familiarity with programming in general.

### Cheatsheet

<table>
   <tr>
      <td><pre>import LIB/default-instruction-set.lm</pre></td>
      <td>Import a Library</td>
   </tr>
   <tr>
      <td><pre>abc</pre></td>
      <td>A Variable</td>
   </tr>
   <tr>
      <td><pre>ABC</pre></td>
      <td>A Literal Value</td>
   </tr>
   <tr>
      <td><pre>'abc</pre></td>
      <td>A Literal Value</td>
   </tr>
   <tr>
      <td><pre>λlhs . rhs</pre></td>
      <td>A Function</td>
   </tr>
   <tr>
      <td><pre>f (a b)</pre></td>
      <td>A Function Application</td>
   </tr>
   <tr>
      <td><pre>f : Arrow&LT;Cons&LT;A,B&GT;,C&GT;&GT;</pre></td>
      <td>A Type Annotation</td>
   </tr>
   <tr>
      <td><pre>f := λlhs . rhs;</pre></td>
      <td>A Global Function Binding</td>
   </tr>
</table>


### Import a Library

Libraries are files relative to the current working directory.

```
import LIB/default-instruction-set.lm;
```

### Tokenization of Syntax

A variable starts with a lower case letter usually.
Anything that isn't a variable will become a Literal.
Variable names can be turned into Literals by prefixing them with a single quote/tick mark: `'abc`.

### Basic Term Syntax

A term is either a Variable, a Literal, a Function Abstraction, a Function Application, or Nil.

Variables and Literals are individual tokens:
```
a    # this is a variable
A    # this is a literal
'a   # this is a literal
```

Function Abstraction follows according to lambda calculus syntax:

```
λ lhs . rhs
```

Function Application follows according to lambda calculus syntax:
```
f arg
```

Terms can always be grouped with parentheses:
```
((λ lhs . rhs) application)
```

Nil is represented as an empty parentheses:
```
()
```

### Extended Term Syntax

Some "functions" are treated special inside the compiler.

```
(sizeof AType)   # get the size of a type as a constant literal number
(: t T)          # ascript a type (hint) onto a term
(as t T)         # cast a term (coerce) into a type
```

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
