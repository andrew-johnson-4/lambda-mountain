# [Lambda Mountain](https://github.com/andrew-johnson-4/-/wiki)

λ☶ (pronounced Lambda Mountain) is a compiler backend that provides a relatively clean implementation of **System F<: with Specialization**.
The whole compiler is only 5000 lines of code, so this could be a good starting point for projects in need of codegen for JIT or AOT compilation.

* [TUTORIAL](https://github.com/andrew-johnson-4/lambda-mountain/blob/main/TUTORIAL/hello-world.md)
* [WIKI](https://github.com/andrew-johnson-4/-/wiki)
* [ASK A QUESTION](https://github.com/andrew-johnson-4/lambda-mountain/discussions/categories/q-a)

### "Portable" Assembler

LM provides abstraction up to the level that you want and down to the level that you want.
Each LM program specifies the exact binary objects to be created, so there is zero room for interpretation by the compiler.
Platform specific defintions and functionality are provided by libraries.
Therefore the compiler has zero hard-coded logic for how to deal with platform architecture.

### Why Such a Small Codebase?

LM solves an N by M Problem with language frontends vs language backends.
The LM project might interface with larger codebases that define frontends or backends, but the core LM Calculus can stay small.

### What is a Fragment Assembler?

An assembler takes pieces of data and sticks them together. Assemblers don't always understand the meaning of what they do, they just do it.

A fragment is a Key-Value Map of Strings to S-Expressions. This data structure permits more detailed manipulation of code than a typical assembler.

### What is Ad-Hoc Specialization?

If we have several overloaded functions then specialization lets us choose the best fit for any particular application.

```
f := λ(: x X). x;
f := λ(: y Y). y;

f (: x X)
```

In this example the function application does not “fit” the application that expects a Y type argument, so there is only one possible candidate function.

---

```
type X implies Y;

f := λ(: x X). x;
f := λ(: y Y). y;

f (: x X)
```

Now both candidate functions “fit”, however X is a narrower type than Y.
All X are Y, but not all Y are X.
In this case we say that X is a “better fit” than Y.

---

```
f := λ(: x X). form 1;
f := λ(: x X). form 2;

f (: x X)
```

In this example both candidate functions “fit” AND are equivalent.
In this case we apply *metrics* to determine the best fit.
A metric is an order that can be applied to term/type pairs to determine which is a “better fit” in non-semantic cases.
Metrics are very useful when there exist multiple equivalent forms of code representation that have different performance characteristics.
Equivalence classes are another high-level concept and are not required to be nominally equivalent.

### Why is Ad-Hoc Specialization so Important For an Assembler?

Specialization allows us to express high-level ideas at the level of a generic functional language
AND compile the code down to machine code transparently.
There are no hidden layers in the compiler.
The programmer gets to inspect and verify *every single transformation down to individual instructions*.

### More About The Type System

The type system is strongly normalizing and decidable as long as all overloaded functions are given explicit types.

Prominent Features include:

* Higher Order Functions (Functional Programming)
* Parametric Polymorphism (Generic Programming)
* Subtyping (Object Hierarchies)
* Ad-Hoc Polymorphism (Function Hierarchies)
* Plural Types (Types behave more like logical predicates)

<a href="https://github.com/andrew-johnson-4/-/wiki#mascot"> <img src="https://raw.githubusercontent.com/andrew-johnson-4/-/main/DOBY.jpg" height=200 title="Doby being a prototypical ass."> </a>

