.global _start
.text
_start:
	jmp main
main:
	push %rbp
	mov %rsp, %rbp
	call uuid_0000000000000004
	movq $123, 0(%r8)
	mov $60, %rax
	mov $0, %rdi
	mov $0, %rsi
	mov $0, %rdx
	syscall
uuid_0000000000000004:
	mov $program_break, %r8
	mov $program_break_counter, %r9
	mov 0(%r9), %r9
	add %r9, %r8
	add $1048576, %r8
	mov $12, %rax
	mov %r8, %rdi
	syscall
	sub $1048574, %r8
	ret
program_break_counter:
	.zero 8
program_break:
