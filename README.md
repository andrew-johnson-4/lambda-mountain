# λ☶. Lambda Mountain
An incredibly hard to read programming language.

# Scope

L1 is polymorphic. λ☶ is monomorphic. That's the core difference.

This programming language implements the `eval-soft` and `eval-hard` functions of [L1's type system](https://github.com/andrew-johnson-4/L1Pearls/blob/main/normalize_kindof.md).
If you want to fully understand or extend L1's core type system then this is the place to start.
Necessarily the L1 bootstrap does not utilize the `eval-*` functions at the type level.
The bootstrap itself should hopefully be fairly legible, but things start to get quirky after that.

`eval-soft` and `eval-hard` are both *exactly equivalent* to well-defined reductions of lambda-calculus terms.
When possible we recommend checking equivalence and/or proving it.
λ☶ is meant only as shorthand for existing techniques for evaluation, testing, optimization, and verification of programs.

# Syntax and Formatting

λ☶ is a programming language written in the passive voice.
`eval-soft` and `eval-hard` are the exposed entry points into your program.
A λ☶ program is then defined as *capabilities* gifted to anyone with access to your entry points.
Each program defines a policy and implementation of capabilities.
Any attempt to subvert these policies should result in an error instead of actual evaluation.

Syntactically, capabilities are exposed to the user as globally bound variables.
User-land programs can then be thought of as simple lambda-calculus expressions.
Below is an example of a user-land program containing one policy bound to the variable `print`.

## print, user-land program
```λ-calculus
print "hello\x20world"
```

## print, policy definition

TBD

# L1 Bootstrap

A definition of a minimal language kernel will be developed in [L1](https://github.com/andrew-johnson-4/LSTS).
L1 itself is not stable yet, so development of this project is delayed pending.
