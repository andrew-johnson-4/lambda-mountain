
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

### Installing LM

First to use LM we should install the compiler on our system.
To build and install LM we can move into the project directory and run `make install`.

```
git clone https://github.com/andrew-johnson-4/lambda-mountain.git
cd lambda-mountain
sudo make install
```

### The simplest LM Program

The simplest program in LM is an empty file.
Let's create a file called `hello_world.lm`.

```
# there is nothing here except for this comment
```

To compile this we should first run lm to produce the assembly file.

```
lm -o hello_world.s hello_world.lm
```

Now the compiler should produce the output `hello_world.s` assembly file.
Let's look inside:


```
.global _start
.text
_start:
        mov $60, %rax
        mov $0, %rdi
        syscall
```

We can see that the code loads the value `$60` into register `%rax`, this corresponds to the Linux x86-64 system call "sys-exit".
The value `$0` into register `%rdi` will indicate a successful exit.
The `syscall` interrupt will invoke the operating system to perform the system call.
This is the shortest possible well-formed program that exits successfully.

