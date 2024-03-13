.global _start
.text
_start:
	jmp main
main:
	mov $45, %rax
	call fib_left_oriented
	mov $60, %rax
	mov $0, %rdi
	syscall
fib_left_oriented:
        cmp $2, %rax
	jle fib_left_oriented_return_one
	push %rax
	sub $1, %rax
	call fib_left_oriented
	pop %r15
	push %rax
	sub $2, %r15
	call fib_right_oriented
	pop %rax
	add %r15, %rax
	ret
fib_left_oriented_return_one:
	mov $1, %rax
	ret
fib_right_oriented:
        cmp $2, %r15
	jle fib_right_oriented_return_one
	push %r15
	sub $1, %r15
	call fib_right_oriented
	pop %rax
	push %r15
	sub $2, %rax
	call fib_left_oriented
	pop %r15
	add %rax, %r15
	ret
fib_right_oriented_return_one:
	mov $1, %r15
	ret
.data
