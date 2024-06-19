
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
.data
hello_world_string:
  .ascii "hello world"
```
