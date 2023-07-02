# λ☶. Lambda Mountain
An incredibly hard to read programming language.

# L1 Bootstrap

A definition of a minimal language kernel will be developed in [L1](https://github.com/andrew-johnson-4/LSTS).
L1 itself is not stable yet, so development of this project is delayed pending.

# Scope

This programming language implements the `eval-soft` and `eval-hard` functions of [L1's type system](https://github.com/andrew-johnson-4/L1Pearls/blob/main/normalize_kindof.md).
If you want to fully understand or extend L1's core type system then this is the place to start.
Necessarily the L1 bootstrap does not utilize the `eval-*` functions at the type level.
The bootstrap itself should hopefully be fairly legible, but things start to get quirky after that.

`eval-soft` and `eval-hard` are both *exactly equivalent* to well-defined reductions of lambda-calculus terms.
When possible we recommend checking equivalence and/or proving it.
λ☶ is meant only as shorthand for existing techniques for evaluation, testing, and verification of programs.
