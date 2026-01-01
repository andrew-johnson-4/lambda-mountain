# [Lambda Mountain](https://github.com/andrew-johnson-4/-/wiki)

λ☶ (pronounced Lambda Mountain) is a typed macro assembler that provides a relatively clean implementation of **System F<: with Specialization**.

* [TUTORIAL](https://github.com/andrew-johnson-4/LSTS/wiki)
* [WIKI](https://github.com/andrew-johnson-4/-/wiki)
* [DOCS](https://lambda-mountain-compiler-backend.github.io/lsts-language-reference/)
* [DISCORD](https://discord.gg/sW2ksPY9jj)

### Not Your Average Assembler

Despite being an assembler, LM provides
* self-hosting (LM is written in LM)
* algebraic data types
* parameterized code and data
* typesafe hygienic macros
* platform agnostic standard libraries
* *à la carte* garbage collection

### Why Such a Small Codebase?

LM is currently about 4000 lines of code.
LM solves an N by M Problem with language frontends vs language backends.
The LM project might interface with larger codebases that define frontends or backends, but the core LM Calculus can stay small.

### What is a Fragment Assembler?

An assembler takes pieces of data and sticks them together. Assemblers don't always understand the meaning of what they do, they just do it.

A fragment is a Key-Value Map of Strings to S-Expressions. This data structure permits more detailed manipulation of code than a typical assembler.

### "Build Your Own Compiler" Development

LM, being a high-level assembler, directly puts the programmer in the position of creating their own custom compiler.
The problem here of course is that compilers are potentially complex and subtle.
To make this more practical, LM leverages cutting-edge techniques to create a more foregiving and approachable experience to developing new compiler features.
This is a problem that is somewhat unique to LM-style development and our community is still learning how to make this more practical.

### LSTS Flavor

The [LSTS](https://github.com/Lambda-Mountain-Compiler-Backend/LSTS) language that LM is developed in is a straightforward extension of C semantics.
You can think of LSTS as just a macro that generates C code.

The LSTS memory model is a restriction of the C memory model with several things such as always-on zero-initialization.

The LSTS expression model is a subset of the C expression model with things that don't behave like expressions altered to just return Nil.

### IDE Support

Currently, there is only Vim and NeoVim syntax highlighting.
[instructions](./vim/README.md)
