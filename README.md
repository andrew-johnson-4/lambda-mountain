# [Lambda Mountain](https://github.com/andrew-johnson-4/-/wiki)

LM aims to provide a convenient interface to formally encode a variety of big-step semantics.
This project is a **User Inferface** for existing theoretical frameworks.
The text language here is a simple encoding of basic lambda calculus and types.
There is also a concept of equivalence relations formed by big-step rules.

<img src="https://raw.githubusercontent.com/andrew-johnson-4/-/main/DOBY.jpg" height=200 title="Doby being a prototypical ass.">

# Installation

LM is currently working towards a bootstrap release.
Until then [Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) are required.

After installing Rust, LM can be installed with

```bash
cargo install lambda_mountain
```

# Syntax and Formatting

In a file `hello_world.lm` put a main function

```
main := (
   print-s hello_world
);
```

Compile `hello_world` from a shell

```
lambda_mountain -o hello_world hello_world.lm
```

Run the result

```
./hello_world
[stdout] hello_world
```

# Mascot

Doby was a donkey that refused to cross a bridge on the way back from a camping trek.
He was left for dead in the rough wilderness with winter approaching.
Somehow he survived the whole winter under that bridge and was discovered by the game warden the next year.
