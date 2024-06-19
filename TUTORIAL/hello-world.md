
### Hello World, from scratch

LM is a fragment assembler, which means it produces object files or flat human-readable assembly.
For this lesson we will be producing human-readable assembly files.

### Desired Output

The goal of this project will be to produce the following file:

```
.global _start
.text
_start:
  jmp main
main:
  mov $1, %rax
  mov $1, %rdi
  mov $hello_world_string, %rsi
  mov $11, %rdx
  syscall
  mov $60, %rax
  mov $0, %rdi
  syscall
.data
hello_world_string:
  .ascii "hello world"
```

This file contains GNU Assembly describing a file with two sections.
The data section contains an ASCII literal value for the string "hello world".
The text section contains a short program that prints the string to STDOUT then exits.

### The simplest LM Program

The simplest program in LM is an empty file.

``` hello_world.lm
# there is nothing here except for this comment
```



