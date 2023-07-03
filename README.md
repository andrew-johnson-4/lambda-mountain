# λ☶. Lambda Mountain

A more idiomatic approach to code lowering.

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

User programs can be any unicode text. Anything. Really. "Free Grammar!"

```λ-calculus
print "hello world"
```

## preprocessing, policy definition

The "string" syntax from the above program needs to be rewritten into a lambda-calculus expression.
Grammatical Rewriting is accomplished in the policy definition as follows.

```λ☶
::pre := λ:" (literal s) :". s
::pre := λc cs. c (::pre cs)
literal := λc (literal cs). c cs
```

## print, policy definition

Each bound variable gets its own line in the policy definition.

```λ☶
print := λmsg. org  0x100 \n (splat msg)  mov  ah, 0x4c \n int  0x21 \n
splat := λc cs. mov dl, c \n mov ah, 2 \n int 0x21 \n (splat cs)
```

## postprocessing, policy definition

The x86 assembler output here needs to be compiled and run.
This is accomplished in the policy definition as follows.

```λ☶
::post := λeval-result. ... side effects go here ...
```

## print, soft-eval reduction

```
soft reductions are not guaranteed to reach normal form.
though, up until they halt they will always be equivalent.
soft reductions will not diverge and are strongly normalizing.
note: "strongly normalizing" does not mean the term is "normalized".
```

## print, hard-eval reduction

```
hard reductions are evaluations until normal form and may diverge.
```

# Rust Bootstrap

TBD.

# L1 Bootstrap

A definition of a minimal language kernel will also be developed in [L1](https://github.com/andrew-johnson-4/LSTS).
L1 itself is not stable yet, so development of this project is delayed pending.
