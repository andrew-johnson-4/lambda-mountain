# λ☶. Lambda Mountain

A more idiomatic approach to code lowering.
λ symbolizes a process for defining the ways to do something.
☶ symbolizes a process for selecting the natural way to do something.
λ answers ["why is this correct?"](https://github.com/andrew-johnson-4/perplexity/blob/main/categorical_prelude.md)
☶ answers ["why is this desirable?"](https://medium.com/@andrew_johnson_4/calligraphy-principles-are-useful-for-proof-construction-e18e9b9a53a5)

# Syntax and Formatting

λ☶ is a programming language written in the passive voice.
`eval-soft` and `eval-hard` are the exposed entry points into your program.
A λ☶ program is then defined as *capabilities* gifted to anyone with access to your entry points.
Each program defines a policy and implementation of capabilities.
Any attempt to subvert these policies should result in an error instead of actual evaluation.

Syntactically, capabilities are exposed to the user as globally bound variables.
User-land programs can then be thought of as simple [lambda-calculus](https://ncatlab.org/nlab/show/lambda-calculus) expressions.

# How is λ☶ different from LSTS

λ☶ is ad-hoc monomorphic. LSTS is ad-hoc polymorphic. 

```λ☶
#λ☶ programs try to apply the first function candidate,
#    followed by the next, in descending order
f := λ(: a A). a
f := λ(: b B). b
(: (f x) A)
(: (f y) B)
```

```LSTS
//LSTS programs try to apply all function candidates,
//     at the same time, immediately
let f(a: A): A = a;
let f(b: B): B = b;
f(x) : A + B
```

# Why is the repo name -

The proper name for this project is λ☶.
However, GitHub disallows most unicode in repo names, probably to avoid lookalike characters in urls.
