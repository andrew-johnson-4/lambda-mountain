# [Lambda Mountain](https://github.com/andrew-johnson-4/-/wiki)

λ☶ (pronounced Lambda Mountain) is a typed macro assembler that provides a relatively clean implementation of **System F<: with Specialization**.

* [TUTORIAL](https://github.com/andrew-johnson-4/LSTS/wiki)
* [WIKI](https://github.com/andrew-johnson-4/-/wiki)
* [DOCS](https://andrew-johnson-4.github.io/lsts-language-reference/)
* [DISCORD](https://discord.gg/sW2ksPY9jj)

### Not Your Average Assembler

Despite being an assembler, LM provides
* self-hosting (LM is written in LM)
* algebraic data types
* parameterized code and data
* hygienic macros
* platform agnostic standard libraries

### Why Such a Small Codebase?

LM is currently about 4000 lines of code.
LM solves an N by M Problem with language frontends vs language backends.
The LM project might interface with larger codebases that define frontends or backends, but the core LM Calculus can stay small.

### What is a Fragment Assembler?

An assembler takes pieces of data and sticks them together. Assemblers don't always understand the meaning of what they do, they just do it.

A fragment is a Key-Value Map of Strings to S-Expressions. This data structure permits more detailed manipulation of code than a typical assembler.

### Example

The LSTS tokenizer is an example of an efficient algorithm implemented in LM.

```
let lsts-tokenize-string(file-path: String, text: String): List<String> = (

   let tokens = [] :: List<String>;
   while non-zero(text) {match text {
      # ignore whitespace
      "\s".. rest => text = rest;
      "\t".. rest => text = rest;
      "\n".. rest => text = rest;

      # consume tokens that start with these strings
      "~=".. rest => (tokens = cons(text[:"~=".length], tokens); text = rest;);
      "+=".. rest => (tokens = cons(text[:"+=".length], tokens); text = rest;);
      ...

      # consume tokens that start with these regular expressions
      (lit=r/^["]([^"\\]|([\\].))*["]/).. rest => (
         tokens = cons(text[:lit.length], tokens); text = rest;
      );
      (rgx=r/^r[\/]([^\/]|([\\].))*[\/]/).. rest => (
         tokens = cons(text[:rgx.length], tokens); text = rest;
      );
      ...

      # otherwise complain about unexpected token
      rest => ( fail("Unrecognized Token in File \{file-path}: \{clone-rope(rest[0])}"); );
   }};

   tokens;
);

# token source location and snippets can be derived from the original source substrings
# this information is only calculated when there is a demand for it
let .token-location(t: String): SourceLocation = (
   let file-path = token-file-paths.lookup( t.data as U64, "[Unknown File]" );
   let line = 1;
   let column = 1;
   let data = t.data;
   while data < t.start {
      if data[0] == $"10_u8" then {
         line = line + 1;
         column = 1;
      } else {
         column = column + 1;
      };
      data = data + 1;
   };
   SourceLocation { file-path, line, column }
);
```

