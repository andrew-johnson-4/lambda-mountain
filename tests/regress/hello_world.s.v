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
