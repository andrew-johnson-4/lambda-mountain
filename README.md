# λ☶. Lambda Mountain

A more idiomatic approach to code lowering.
λ symbolizes a process for defining the ways to do something.
☶ symbolizes a process for selecting the natural way to do something.
λ answers ["why is this correct?"](https://github.com/andrew-johnson-4/PunCalculus)
☶ answers ["why is this desirable?"](https://medium.com/@andrew_johnson_4/calligraphy-principles-are-useful-for-proof-construction-e18e9b9a53a5)

# Contribution

λ☶ aims to provide a convenient interface to formally encode a variety of big-step semantics.
This project is a **User Inferface** for existing theoretical frameworks.
The text language here is a simple encoding of basic lambda calculus and types.
Future work will also be done to provide a zoom-in/zoom-out interface for rules.

# Syntax and Formatting

λ☶ is a programming language written in the passive voice.
`eval-soft` and `eval-hard` are the exposed entry points into your program.
A λ☶ program is then defined as *capabilities* gifted to anyone with access to your entry points.
Each program defines a policy and implementation of capabilities.
Any attempt to subvert these policies should result in an error instead of actual evaluation.

Syntactically, capabilities are exposed to the user as globally bound variables.
User-land programs can then be thought of as simple [lambda-calculus](https://ncatlab.org/nlab/show/lambda-calculus) expressions.

![Equation](https://github.com/andrew-johnson-4/-/blob/main/equation.png)

# How is eval-soft different from eval-hard?

_eval-soft_ attempts to evaluate an expression to normal form with two restrictions:
* the evaluation must not diverge
* the result must be referentially transparent

_eval-hard_ does not have any restrictions.

# Why is the repo name -

The proper name for this project is λ☶.
However, GitHub disallows most unicode in repo names, probably to avoid lookalike characters in urls.
