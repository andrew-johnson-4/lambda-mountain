# [Lambda Mountain](https://github.com/andrew-johnson-4/-/wiki)

λ☶ (pronounced Lambda Mountain) is a typed macro assembler that provides a relatively clean implementation of **System F<: with Specialization**.

* [TUTORIAL](https://github.com/andrew-johnson-4/lambda-mountain/blob/main/TUTORIAL/hello-blob.md)
* [WIKI](https://github.com/andrew-johnson-4/-/wiki)
* [DOCS](https://andrew-johnson-4.github.io/lambda-mountain/)

### Not Your Average Assembler

Despite being an assembler, LM provides
* self-hosting (LM is written in LM)
* algebraic data types
* parameterized code and data
* hygienic macros
* platform agnostic standard libraries

### Why Such a Small Codebase?

LM is currenlty about 4000 lines of code.
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

### Why is Ad-Hoc Specialization so Important For an Assembler?

Specialization allows us to express high-level ideas at the level of a generic functional language
AND compile the code down to target objects transparently.
There are no hidden layers in the compiler.
The programmer gets to inspect and verify *every single transformation down to individual instructions*.

### More About The Type System

The type system is strongly normalizing and decidable as long as all overloaded functions are given acceptable explicit types.

Prominent Features include:

* Higher Order Functions (Functional Programming)
* Parametric Polymorphism (Generic Programming)
* Subtyping (Object Hierarchies)
* Ad-Hoc Polymorphism (Function Hierarchies)
* Plural Types (Types behave more like logical predicates)

<a href="https://github.com/andrew-johnson-4/-/wiki#mascot"> <img src="https://raw.githubusercontent.com/andrew-johnson-4/-/main/DOBY.jpg" height=200 title="Doby being a prototypical ass."> </a>

