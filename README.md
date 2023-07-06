# λ☶. Lambda Mountain

A more idiomatic approach to code lowering.
λ symbolizes a process for defining the ways to do something.
☶ symbolizes a process for selecting the natural way to do something.
λ answers "why is this correct?"
☶ answers "why is this desirable?"

# Syntax and Formatting

λ☶ is a programming language written in the passive voice.
`eval-soft` and `eval-hard` are the exposed entry points into your program.
A λ☶ program is then defined as *capabilities* gifted to anyone with access to your entry points.
Each program defines a policy and implementation of capabilities.
Any attempt to subvert these policies should result in an error instead of actual evaluation.

Syntactically, capabilities are exposed to the user as globally bound variables.
User-land programs can then be thought of as simple [lambda-calculus](https://ncatlab.org/nlab/show/lambda-calculus) expressions.
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
::pre := λ["] (literal s) ["]. s
::pre := λc (::pre cs). c cs
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
soft reductions will not diverge and are strongly normalizing.
if a term may diverge then an error may occur.
blame is defined by the rewrite rules.
```

## print, hard-eval reduction

```
hard reductions are evaluations until normal form and may diverge.
```

## print, running the program

```bash
lambda_mountain print.lm < print.txt
```

# Intelligent Reduction

The magic really starts to happen when we connect the above term definitions with *intelligent equivalences*.
Equivalences are defined by [rewrite rules](https://en.wikipedia.org/wiki/Type_theory#Rules) that look like this:

<img src="https://github.com/andrew-johnson-4/-/blob/main/4487ca46bc4413415a8ccc0820eddb8978a06a81.svg" alt="lambda introduction" width=40%>

# Calligraphy

Stylistic considerations are important when proof trees start to look like this:

<img src="https://github.com/andrew-johnson-4/-/blob/main/calligraphy.png" alt="proof tree" width=40%>

# Why is the repo name -

The proper name for this project is λ☶.
However, Github disallows most unicode in repo names, probably to avoid lookalike characters in urls.
