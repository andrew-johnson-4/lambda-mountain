
### Automating Formal Verification With LM Verifier And Coq Theorem Prover

When LM code is compiled there is an option to produce verifiable GNU assembly.
When assembly is annotated this way it can be verified with the `lmv` verifier and some assistance from the Coq Theorem Prover.
The lmv tool produces a Coq proof that can be verified automatically.
This proof models things such as memory safety and is provable for most LM code including the standard library.

For starters we will verify the hello world program.
We can slightly modify our previous hello world program to terminate with two conditions.
* "Hello World\n" should be printed to STDOUT.
* The program should quietly exit with status SUCCESS(0)

```
.global _start
.text
_start:
        push %rbp
        mov %rsp, %rbp
        call main
        mov %rbp, %rsp
        pop %rbp
        mov $60, %rax
        mov $0, %rdi
        syscall
#expect STDOUT.ends_with("Hello World\n")
#expect EXIT_STATUS=0
main:
        mov $1, %RAX
        mov $1, %RDI
        mov $uuid_0000000000007838, %RSI
        mov $11, %RDX
        syscall
        mov %rbp, %rsp
        sub $8, %rsp
        ret
.data
uuid_0000000000007838:
        .ascii "Hello World\n"
        .zero 1
```

We add these two post-conditions with an `expect` comment in the assembly file.
However, if we look closely at the program you may notice that there is also a Trojan Horse.
We have slighly changed the data message from our previous example from "hello_world" to "Hello World\n".
Both the string constant AND the expected output have been changed correctly, however we have forgotten to update the system call.
The system call is still configured to print 11 characters of output instead of the new 12 byte "Hello World\n" that includes a newline character.

Sneaky! Let's see what happens when we try to verify this.

### References
* [Using Coq as an Assembler](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/12/coqasm.pdf)
  Previous work has already been directed at modelling assembler in Coq.
  However there were some restrictions in the work, namely you had to learn Coq.
* [An encoding of X86 in Coq](https://github.com/maximedenes/coq-amd64)
  This project encodes some x86 instructions in Coq, however it does not do much for interpreting the meaning of those instructions.
  It looks like this project is again more tuned towards using Coq as a macro assembler instead of extending the assembler itself.
