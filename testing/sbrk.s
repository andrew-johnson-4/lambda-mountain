.global _start
.text
_start:
	jmp main
main:
	call allocate_100
	movq $123, 0(%r8)
	movq $123, 16(%r8)
	mov %rax, %rdi
	mov $60, %rax
	syscall
allocate_100:
	mov $program_break, %r8
	mov %r8, %rdi
	add $100, %rdi
	mov $12, %rax
	syscall
	ret
.data
current_break:
	.zero 8
program_break:
