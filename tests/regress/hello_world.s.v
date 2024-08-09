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
main:
#Call Fragment mov : Arrow<Cons<Constant+Literal+Sized<8>,Constant+Reg64>,Nil>
	mov $1, %RAX
#Call Fragment mov : Arrow<Cons<Constant+Literal+Sized<8>,Constant+Reg64>,Nil>
	mov $1, %RDI
#Call Fragment mov : Arrow<Cons<Constant+Literal+Sized<8>,Constant+Reg64>,Nil>
	mov $uuid_0000000000007838, %RSI
#Call Fragment mov : Arrow<Cons<Constant+Literal+Sized<8>,Constant+Reg64>,Nil>
	mov $11, %RDX
#Call Fragment syscall : Arrow<Nil,Nil>
	syscall
	mov %rbp, %rsp
	sub $8, %rsp
	ret
.data
uuid_0000000000007838:
	.ascii "hello_world"
	.zero 1
