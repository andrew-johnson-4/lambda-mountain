 .global _start 
.text 
_start: 
	jmp main 
_strlen: 
	xor %rbx, %rbx 
_strlen_loop: 
	cmpb $0, 0(%rax) 
	jz _strlen_exit 
	inc %rax 
	inc %rbx 
	jmp _strlen_loop 
_strlen_exit: 
	ret 
print_i: 
	mov %r12, %r8 
	push %rsi 
	push %rdi 
	push %rax 
	push %rbx 
	push %rcx 
	push %rdx 
	push %r8 
	push %r9 
	push %r10 
	mov $__put64_write_buffer, %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	mov $1, %rax 
	mov $1, %rdi 
	mov $__put64_buffer, %rsi 
	mov $19, %rdx 
	syscall 
	pop %r10 
	pop %r9 
	pop %r8 
	pop %rdx 
	pop %rcx 
	pop %rbx 
	pop %rax 
	pop %rdi 
	pop %rsi 
	ret 
print_p: 
	mov 0(%r12), %r8 
	push %rsi 
	push %rdi 
	push %rax 
	push %rbx 
	push %rcx 
	push %rdx 
	push %r8 
	push %r9 
	push %r10 
	mov $__put64_write_buffer, %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	mov $1, %rax 
	mov $1, %rdi 
	mov $__put64_buffer, %rsi 
	mov $19, %rdx 
	syscall 
	pop %r10 
	pop %r9 
	pop %r8 
	pop %rdx 
	pop %rcx 
	pop %rbx 
	pop %rax 
	pop %rdi 
	pop %rsi 
	ret 
print_s: 
	mov $0, %r9 
	jmp __print_this 
print_d: 
	mov $1, %r9 
	jmp __print_this 
__print_this: 
	push %rax 
	push %rbx 
	push %rcx 
	push %rdx 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	push %rsi 
	push %rdi 
	cmp $0, %r9 
	je __print_clear 
	mov %r12, %r8 
	push %rsi 
	push %rdi 
	push %rax 
	push %rbx 
	push %rcx 
	push %rdx 
	push %r8 
	push %r9 
	push %r10 
	mov $__put64_write_buffer, %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	mov $1, %rax 
	mov $1, %rdi 
	mov $__put64_buffer, %rsi 
	mov $19, %rdx 
	syscall 
	pop %r10 
	pop %r9 
	pop %r8 
	pop %rdx 
	pop %rcx 
	pop %rbx 
	pop %rax 
	pop %rdi 
	pop %rsi 
	mov %r13, %r8 
	push %rsi 
	push %rdi 
	push %rax 
	push %rbx 
	push %rcx 
	push %rdx 
	push %r8 
	push %r9 
	push %r10 
	mov $__put64_write_buffer, %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	mov $1, %rax 
	mov $1, %rdi 
	mov $__put64_buffer, %rsi 
	mov $19, %rdx 
	syscall 
	pop %r10 
	pop %r9 
	pop %r8 
	pop %rdx 
	pop %rcx 
	pop %rbx 
	pop %rax 
	pop %rdi 
	pop %rsi 
	mov %r14, %r8 
	push %rsi 
	push %rdi 
	push %rax 
	push %rbx 
	push %rcx 
	push %rdx 
	push %r8 
	push %r9 
	push %r10 
	mov $__put64_write_buffer, %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	mov $1, %rax 
	mov $1, %rdi 
	mov $__put64_buffer, %rsi 
	mov $19, %rdx 
	syscall 
	pop %r10 
	pop %r9 
	pop %r8 
	pop %rdx 
	pop %rcx 
	pop %rbx 
	pop %rax 
	pop %rdi 
	pop %rsi 
	mov %r15, %r8 
	push %rsi 
	push %rdi 
	push %rax 
	push %rbx 
	push %rcx 
	push %rdx 
	push %r8 
	push %r9 
	push %r10 
	mov $__put64_write_buffer, %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	rol $4, %r8 
	mov %r8b, %al 
	and $0xf, %al 
	mov $__hex_buffer, %r10 
	add %al, %r10b 
	mov 0(%r10), %cl 
	mov %cl, 0(%r9) 
	inc %r9 
	mov $1, %rax 
	mov $1, %rdi 
	mov $__put64_buffer, %rsi 
	mov $19, %rdx 
	syscall 
	pop %r10 
	pop %r9 
	pop %r8 
	pop %rdx 
	pop %rcx 
	pop %rbx 
	pop %rax 
	pop %rdi 
	pop %rsi 
__print_clear: 
__print_this_atom: 
	cmp $0, %r12 
	je __print_this_cons 
	mov %r12, %rax 
	call _strlen 
	mov %r12, %rsi 
	mov %rbx, %rdx 
	mov $1, %rax 
	mov $1, %rdi 
	syscall 
	pop %rdi 
	pop %rsi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	pop %rdx 
	pop %rcx 
	pop %rbx 
	pop %rax 
	ret 
__print_this_cons: 
	cmp $0, %r13 
	je __print_this_nil 
	cmp $0, %r14 
	je __print_this_nil 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	push %rsi 
	push %rdi 
	mov $1, %rax 
	mov $1, %rdi 
	mov $__lparen, %rsi 
	mov $1, %rdx 
	syscall 
	pop %rdi 
	pop %rsi 
	mov %r13, %rsi 
	mov 0(%rsi), %r12 
	mov 8(%rsi), %r13 
	mov 16(%rsi), %r14 
	mov 24(%rsi), %r15 
	call __print_this 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	push %rsi 
	push %rdi 
	mov $1, %rax 
	mov $1, %rdi 
	mov $__space, %rsi 
	mov $1, %rdx 
	syscall 
	pop %rdi 
	pop %rsi 
	mov %r14, %rsi 
	mov 0(%rsi), %r12 
	mov 8(%rsi), %r13 
	mov 16(%rsi), %r14 
	mov 24(%rsi), %r15 
	call __print_this 
	push %rsi 
	push %rdi 
	mov $1, %rax 
	mov $1, %rdi 
	mov $__rparen, %rsi 
	mov $1, %rdx 
	syscall 
	pop %rdi 
	pop %rsi 
	pop %rdi 
	pop %rsi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	pop %rdx 
	pop %rcx 
	pop %rbx 
	pop %rax 
	ret 
__print_this_nil: 
	mov $1, %rax 
	mov $1, %rdi 
	mov $__nil_literal, %rsi 
	mov $2, %rdx 
	syscall 
	pop %rdi 
	pop %rsi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	pop %rdx 
	pop %rcx 
	pop %rbx 
	pop %rax 
	ret 
head: 
	cmp $0, %r13 
	je __head_is_nil 
	mov 0(%r13), %r12 
	mov 16(%r13), %r14 
	mov 24(%r13), %r15 
	mov 8(%r13), %r13 
	ret 
__head_is_nil: 
	mov $0, %r12 
	mov $0, %r13 
	mov $0, %r14 
	mov $0, %r15 
	ret 
tail: 
	cmp $0, %r14 
	je __tail_is_nil 
	mov 0(%r14), %r12 
	mov 24(%r14), %r15 
	mov 8(%r14), %r13 
	mov 16(%r14), %r14 
	ret 
__tail_is_nil: 
	mov $0, %r12 
	mov $0, %r13 
	mov $0, %r14 
	mov $0, %r15 
	ret 
not: 
	push %rax 
	push %rbx 
	push %rcx 
	push %rdx 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	push %rsi 
	push %rdi 
	cmp $0, %r12 
	jne __not_is_some 
	cmp $0, %r13 
	jne __not_is_some 
	cmp $0, %r14 
	jne __not_is_some 
	pop %rdi 
	pop %rsi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	pop %rdx 
	pop %rcx 
	pop %rbx 
	pop %rax 
	mov $ __true , %r12 
	mov $0, %r13 
	mov $0, %r14 
	mov $0, %r15 
	ret 
__not_is_some: 
	pop %rdi 
	pop %rsi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	pop %rdx 
	pop %rcx 
	pop %rbx 
	pop %rax 
	mov $0, %r12 
	mov $0, %r13 
	mov $0, %r14 
	mov $0, %r15 
	ret 
eq: 
	cmp $0, %r13 
	je __equal_nil 
	cmp $0, %r14 
	je __equal_nil 
	mov 0(%r13), %rax 
	mov 0(%r14), %rbx 
	cmp $0, %rax 
	je __equal_nil 
	cmp $0, %rbx 
	je __equal_nil 
	call _streq 
	ret 
__equal_nil: 
	mov $0, %r12 
	mov $0, %r13 
	mov $0, %r14 
	mov $0, %r15 
	ret 
_streq: 
__streq_loop: 
	cmp $0, %rax 
	je __streq_false 
	cmp $0, %rbx 
	je __streq_false 
	mov 0(%rax), %cl 
	mov 0(%rbx), %dl 
	cmp %cl, %dl 
	jne __streq_false 
	cmp $0, %cl 
	je __streq_true 
	inc %rax 
	inc %rbx 
	jmp __streq_loop 
__streq_true: 
	mov $__true, %r12 
	mov $0, %r13 
	mov $0, %r14 
	mov $0, %r15 
	ret 
__streq_false: 
	mov $0, %r12 
	mov $0, %r13 
	mov $0, %r14 
	mov $0, %r15 
	ret 
clone_rope: 
	mov $__a_section, %r8 
	mov $__a_counter, %r10 
	mov 0(%r10), %r11 
	add %r11, %r8 
	mov %r8, %r9 
	call __clone_rope 
	movb $0, 0(%r9) 
	inc %r9 
	mov $__a_section, %r10 
	sub %r10, %r9 
	mov $__a_counter, %r10 
	mov %r9, 0(%r10) 
	mov %r8, %r12 
	mov $0, %r13 
	mov $0, %r14 
	mov $0, %r15 
	ret 
__clone_rope: 
	cmp $0, %r13 
	je clone_rope_notcons 
	cmp $0, %r14 
	je clone_rope_notcons 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov 0(%r13), %r12 
	mov 16(%r13), %r14 
	mov 24(%r13), %r15 
	mov 8(%r13), %r13 
	call __clone_rope 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov 0(%r14), %r12 
	mov 24(%r14), %r15 
	mov 8(%r14), %r13 
	mov 16(%r14), %r14 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	call __clone_rope 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	jmp clone_rope_end 
clone_rope_notcons: 
	cmp $0, %r12 
	je clone_rope_end 
clone_rope_small: 
	cmpb $0, 0(%r12) 
	je clone_rope_end 
	movb 0(%r12), %bl 
	movb %bl, 0(%r9) 
	inc %r12 
	inc %r9 
	jmp clone_rope_small 
clone_rope_end: 
	ret 
write_file: 
	mov 0(%r13), %rdi 
	pushq 0(%r14) 
	mov $2, %rax 
	mov $65, %rsi 
	mov $420, %rdx 
	syscall 
	mov %rax, %r8 
	pop %rax 
	mov %rax, %r9 
	call _strlen 
	mov %rbx, %rdx 
	mov %r9, %rsi 
	mov %r8, %rdi 
	mov $1, %rax 
	syscall 
	mov %r8, %rdi 
	mov $3, %rax 
	syscall 
	ret 
load_file: 
	mov $2, %rax 
	mov %r12, %rdi 
	mov $0, %rsi 
	syscall 
	cmp $0, %rax 
	jge load_file_contents 
	mov $__err_fopen, %r12 
	mov $0, %r13 
	mov $0, %r14 
	mov $0, %r15 
	ret 
load_file_contents: 
	mov $__a_section, %r8 
	mov $__a_counter, %r10 
	mov 0(%r10), %r11 
	add %r11, %r8 
	mov %r8, %r9 
	mov $0, %r10 
	mov $load_file_buf, %r11 
load_file_loop: 
	cmp $0, %r10 
	je load_file_bufempty 
	movb 0(%r11), %bl 
	mov %bl, 0(%r9) 
	inc %r9 
	inc %r11 
	dec %r10 
	jmp load_file_loop 
load_file_bufempty: 
	push %rax 
	mov %rax, %rdi 
	mov $0, %rax 
	mov $load_file_buf, %rsi 
	mov $load_file_bsz, %rdx 
	mov 0(%rdx), %rdx 
	syscall 
	mov %rax, %r10 
	pop %rax 
	mov $load_file_buf, %r11 
	cmp $0, %r10 
	jne load_file_loop 
	mov %rax, %rdi 
	mov $3, %rax 
	syscall 
	movb $0, 0(%r9) 
	inc %r9 
	sub %r8, %r9 
	mov $__a_counter, %r10 
	mov %r9, 0(%r10) 
	mov %r8, %r12 
	mov $0, %r13 
	mov $0, %r14 
	mov $0, %r15 
	ret 
as: 
	call print_s 
	ret 
destructure:
	push %rbp 
	mov %rsp, %rbp 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	mov %r12, %r8 
	mov %r13, %r9 
	mov %r14, %r10 
	mov %r15, %r11 
	call tail 
	mov %r12, -8(%rbp)
 	mov %r13, -16(%rbp)
 	mov %r14, -24(%rbp)
 	mov %r15, -32(%rbp)
	mov %r8, %r12 
	mov %r9, %r13 
	mov %r10, %r14 
	mov %r11, %r15 
	call head 
	mov %r12, %r8 
	mov %r13, %r9 
	mov %r14, %r10 
	mov %r15, %r11 
	call tail 
	mov %r12, -40(%rbp)
 	mov %r13, -48(%rbp)
 	mov %r14, -56(%rbp)
 	mov %r15, -64(%rbp)
	mov %r8, %r12 
	mov %r9, %r13 
	mov %r10, %r14 
	mov %r11, %r15 
	call head 
	mov %r12, -72(%rbp)
 	mov %r13, -80(%rbp)
 	mov %r14, -88(%rbp)
 	mov %r15, -96(%rbp)
	mov -72(%rbp), %r12
 	mov -80(%rbp), %r13
 	mov -88(%rbp), %r14
 	mov -96(%rbp), %r15
	cmp $0, %r12
	jne _uuid_15
	cmp $0, %r13
	jne _uuid_15
	cmp $0, %r14
	jne _uuid_15
	cmp $0, %r15
	jne _uuid_15
	mov $0, %r12 
	mov $0, %r13 
	mov $0, %r14 
	mov $0, %r15 
	jmp _uuid_16
_uuid_15:
	mov -40(%rbp), %r12
 	mov -48(%rbp), %r13
 	mov -56(%rbp), %r14
 	mov -64(%rbp), %r15
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov -8(%rbp), %r12
 	mov -16(%rbp), %r13
 	mov -24(%rbp), %r14
 	mov -32(%rbp), %r15
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov $0, %r12 
	mov %rsi, %r13 
	mov %rdi, %r14 
	mov $0, %r15 
	mov $0, %rsi
	cmp $0, %rsi
	jne _uuid_5
	mov $0, %rsi
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	cmp $0, %r13
	je _uuid_1
	mov 0(%r13), %r12 
	mov 16(%r13), %r14 
	mov 24(%r13), %r15 
	mov 8(%r13), %r13 
	mov $0, %rsi
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	cmp $0, %r13
	je _uuid_2
	mov 0(%r13), %r12 
	mov 16(%r13), %r14 
	mov 24(%r13), %r15 
	mov 8(%r13), %r13 
	mov %r12, %r8 
	mov %r13, %r9 
	mov %r14, %r10 
	mov %r15, %r11 
	mov %r12, %rax
	mov $ _uuid_4 , %r12 
	mov $0, %r13 
	mov $0, %r14 
	mov $0, %r15 
	mov %r12, %rbx
	call _streq
	mov %r12, %rdi
	mov %r8, %r12 
	mov %r9, %r13 
	mov %r10, %r14 
	mov %r11, %r15 
	cmp $0, %rdi
	je _uuid_3
	mov $1, %rsi
_uuid_3:
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	cmp $0, %rsi
	je _uuid_2
	mov $0, %rsi
	cmp $0, %r14
	je _uuid_2
	mov 0(%r14), %r12 
	mov 24(%r14), %r15 
	mov 8(%r14), %r13 
	mov 16(%r14), %r14 
	mov %r12, -104(%rbp)
 	mov %r13, -112(%rbp)
 	mov %r14, -120(%rbp)
 	mov %r15, -128(%rbp)
	mov $1, %rsi
_uuid_2:
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	cmp $0, %rsi
	je _uuid_1
	mov $0, %rsi
	cmp $0, %r14
	je _uuid_1
	mov 0(%r14), %r12 
	mov 24(%r14), %r15 
	mov 8(%r14), %r13 
	mov 16(%r14), %r14 
	mov %r12, -136(%rbp)
 	mov %r13, -144(%rbp)
 	mov %r14, -152(%rbp)
 	mov %r15, -160(%rbp)
	mov $1, %rsi
_uuid_1:
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	cmp $0, %rsi
	je _uuid_5
	mov -72(%rbp), %r12
 	mov -80(%rbp), %r13
 	mov -88(%rbp), %r14
 	mov -96(%rbp), %r15
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov -104(%rbp), %r12
 	mov -112(%rbp), %r13
 	mov -120(%rbp), %r14
 	mov -128(%rbp), %r15
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov -136(%rbp), %r12
 	mov -144(%rbp), %r13
 	mov -152(%rbp), %r14
 	mov -160(%rbp), %r15
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov $0, %r12 
	mov %rsi, %r13 
	mov %rdi, %r14 
	mov $0, %r15 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov $0, %r12 
	mov %rsi, %r13 
	mov %rdi, %r14 
	mov $0, %r15 
	mov $1, %rsi
_uuid_5:
	cmp $0, %rsi
	jne _uuid_9
	mov $0, %rsi
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	cmp $0, %r13
	je _uuid_6
	mov 0(%r13), %r12 
	mov 16(%r13), %r14 
	mov 24(%r13), %r15 
	mov 8(%r13), %r13 
	mov $0, %rsi
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	cmp $0, %r13
	je _uuid_7
	mov 0(%r13), %r12 
	mov 16(%r13), %r14 
	mov 24(%r13), %r15 
	mov 8(%r13), %r13 
	mov %r12, -168(%rbp)
 	mov %r13, -176(%rbp)
 	mov %r14, -184(%rbp)
 	mov %r15, -192(%rbp)
	mov $1, %rsi
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	cmp $0, %rsi
	je _uuid_7
	mov $0, %rsi
	cmp $0, %r14
	je _uuid_7
	mov 0(%r14), %r12 
	mov 24(%r14), %r15 
	mov 8(%r14), %r13 
	mov 16(%r14), %r14 
	mov %r12, -200(%rbp)
 	mov %r13, -208(%rbp)
 	mov %r14, -216(%rbp)
 	mov %r15, -224(%rbp)
	mov $1, %rsi
_uuid_7:
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	cmp $0, %rsi
	je _uuid_6
	mov $0, %rsi
	cmp $0, %r14
	je _uuid_6
	mov 0(%r14), %r12 
	mov 24(%r14), %r15 
	mov 8(%r14), %r13 
	mov 16(%r14), %r14 
	mov $0, %rsi
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	cmp $0, %r13
	je _uuid_8
	mov 0(%r13), %r12 
	mov 16(%r13), %r14 
	mov 24(%r13), %r15 
	mov 8(%r13), %r13 
	mov %r12, -232(%rbp)
 	mov %r13, -240(%rbp)
 	mov %r14, -248(%rbp)
 	mov %r15, -256(%rbp)
	mov $1, %rsi
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	cmp $0, %rsi
	je _uuid_8
	mov $0, %rsi
	cmp $0, %r14
	je _uuid_8
	mov 0(%r14), %r12 
	mov 24(%r14), %r15 
	mov 8(%r14), %r13 
	mov 16(%r14), %r14 
	mov %r12, -264(%rbp)
 	mov %r13, -272(%rbp)
 	mov %r14, -280(%rbp)
 	mov %r15, -288(%rbp)
	mov $1, %rsi
_uuid_8:
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
_uuid_6:
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	cmp $0, %rsi
	je _uuid_9
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov -72(%rbp), %r12
 	mov -80(%rbp), %r13
 	mov -88(%rbp), %r14
 	mov -96(%rbp), %r15
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov -168(%rbp), %r12
 	mov -176(%rbp), %r13
 	mov -184(%rbp), %r14
 	mov -192(%rbp), %r15
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov $0, %r12 
	mov %rsi, %r13 
	mov %rdi, %r14 
	mov $0, %r15 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov -232(%rbp), %r12
 	mov -240(%rbp), %r13
 	mov -248(%rbp), %r14
 	mov -256(%rbp), %r15
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov $0, %r12 
	mov %rsi, %r13 
	mov %rdi, %r14 
	mov $0, %r15 
	call destructure 
	mov %r12, -296(%rbp)
 	mov %r13, -304(%rbp)
 	mov %r14, -312(%rbp)
 	mov %r15, -320(%rbp)
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov $0, %r12 
	mov %rsi, %r13 
	mov %rdi, %r14 
	mov $0, %r15 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov -296(%rbp), %r12
 	mov -304(%rbp), %r13
 	mov -312(%rbp), %r14
 	mov -320(%rbp), %r15
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov -200(%rbp), %r12
 	mov -208(%rbp), %r13
 	mov -216(%rbp), %r14
 	mov -224(%rbp), %r15
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov $0, %r12 
	mov %rsi, %r13 
	mov %rdi, %r14 
	mov $0, %r15 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov -264(%rbp), %r12
 	mov -272(%rbp), %r13
 	mov -280(%rbp), %r14
 	mov -288(%rbp), %r15
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov $0, %r12 
	mov %rsi, %r13 
	mov %rdi, %r14 
	mov $0, %r15 
	call destructure 
	mov %r12, -296(%rbp)
 	mov %r13, -304(%rbp)
 	mov %r14, -312(%rbp)
 	mov %r15, -320(%rbp)
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov $0, %r12 
	mov %rsi, %r13 
	mov %rdi, %r14 
	mov $0, %r15 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov -296(%rbp), %r12
 	mov -304(%rbp), %r13
 	mov -312(%rbp), %r14
 	mov -320(%rbp), %r15
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov $0, %r12 
	mov %rsi, %r13 
	mov %rdi, %r14 
	mov $0, %r15 
	call tail 
	mov $1, %rsi
_uuid_9:
	cmp $0, %rsi
	jne _uuid_13
	mov $0, %rsi
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	cmp $0, %r13
	je _uuid_10
	mov 0(%r13), %r12 
	mov 16(%r13), %r14 
	mov 24(%r13), %r15 
	mov 8(%r13), %r13 
	mov %r12, -328(%rbp)
 	mov %r13, -336(%rbp)
 	mov %r14, -344(%rbp)
 	mov %r15, -352(%rbp)
	mov $1, %rsi
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	cmp $0, %rsi
	je _uuid_10
	mov $0, %rsi
	cmp $0, %r14
	je _uuid_10
	mov 0(%r14), %r12 
	mov 24(%r14), %r15 
	mov 8(%r14), %r13 
	mov 16(%r14), %r14 
	mov %r12, -360(%rbp)
 	mov %r13, -368(%rbp)
 	mov %r14, -376(%rbp)
 	mov %r15, -384(%rbp)
	mov $1, %rsi
_uuid_10:
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	cmp $0, %rsi
	je _uuid_13
	mov -328(%rbp), %r12
 	mov -336(%rbp), %r13
 	mov -344(%rbp), %r14
 	mov -352(%rbp), %r15
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov -360(%rbp), %r12
 	mov -368(%rbp), %r13
 	mov -376(%rbp), %r14
 	mov -384(%rbp), %r15
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov $0, %r12 
	mov %rsi, %r13 
	mov %rdi, %r14 
	mov $0, %r15 
	call eq 
	cmp $0, %r12
	jne _uuid_11
	cmp $0, %r13
	jne _uuid_11
	cmp $0, %r14
	jne _uuid_11
	cmp $0, %r15
	jne _uuid_11
	mov $0, %r12 
	mov $0, %r13 
	mov $0, %r14 
	mov $0, %r15 
	jmp _uuid_12
_uuid_11:
	mov -72(%rbp), %r12
 	mov -80(%rbp), %r13
 	mov -88(%rbp), %r14
 	mov -96(%rbp), %r15
_uuid_12:
	mov $1, %rsi
_uuid_13:
	cmp $0, %rsi
	jne _uuid_14
	mov $0, %r12 
	mov $0, %r13 
	mov $0, %r14 
	mov $0, %r15 
_uuid_14:
_uuid_16:
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	mov %rbp, %rsp 
	pop %rbp 
	ret 
ctx_eval_soft:
	push %rbp 
	mov %rsp, %rbp 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	mov %r12, %r8 
	mov %r13, %r9 
	mov %r14, %r10 
	mov %r15, %r11 
	call tail 
	mov %r12, -8(%rbp)
 	mov %r13, -16(%rbp)
 	mov %r14, -24(%rbp)
 	mov %r15, -32(%rbp)
	mov %r8, %r12 
	mov %r9, %r13 
	mov %r10, %r14 
	mov %r11, %r15 
	call head 
	mov %r12, -40(%rbp)
 	mov %r13, -48(%rbp)
 	mov %r14, -56(%rbp)
 	mov %r15, -64(%rbp)
	mov -8(%rbp), %r12
 	mov -16(%rbp), %r13
 	mov -24(%rbp), %r14
 	mov -32(%rbp), %r15
	mov $0, %rsi
	cmp $0, %rsi
	jne _uuid_22
	mov $0, %rsi
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	cmp $0, %r13
	je _uuid_17
	mov 0(%r13), %r12 
	mov 16(%r13), %r14 
	mov 24(%r13), %r15 
	mov 8(%r13), %r13 
	mov $0, %rsi
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	cmp $0, %r13
	je _uuid_18
	mov 0(%r13), %r12 
	mov 16(%r13), %r14 
	mov 24(%r13), %r15 
	mov 8(%r13), %r13 
	mov %r12, %r8 
	mov %r13, %r9 
	mov %r14, %r10 
	mov %r15, %r11 
	mov %r12, %rax
	mov $ _uuid_20 , %r12 
	mov $0, %r13 
	mov $0, %r14 
	mov $0, %r15 
	mov %r12, %rbx
	call _streq
	mov %r12, %rdi
	mov %r8, %r12 
	mov %r9, %r13 
	mov %r10, %r14 
	mov %r11, %r15 
	cmp $0, %rdi
	je _uuid_19
	mov $1, %rsi
_uuid_19:
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	cmp $0, %rsi
	je _uuid_18
	mov $0, %rsi
	cmp $0, %r14
	je _uuid_18
	mov 0(%r14), %r12 
	mov 24(%r14), %r15 
	mov 8(%r14), %r13 
	mov 16(%r14), %r14 
	mov %r12, -72(%rbp)
 	mov %r13, -80(%rbp)
 	mov %r14, -88(%rbp)
 	mov %r15, -96(%rbp)
	mov $1, %rsi
_uuid_18:
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	cmp $0, %rsi
	je _uuid_17
	mov $0, %rsi
	cmp $0, %r14
	je _uuid_17
	mov 0(%r14), %r12 
	mov 24(%r14), %r15 
	mov 8(%r14), %r13 
	mov 16(%r14), %r14 
	mov %r12, -104(%rbp)
 	mov %r13, -112(%rbp)
 	mov %r14, -120(%rbp)
 	mov %r15, -128(%rbp)
	mov $1, %rsi
_uuid_17:
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	cmp $0, %rsi
	je _uuid_22
	mov $ _uuid_21 , %r12 
	mov $0, %r13 
	mov $0, %r14 
	mov $0, %r15 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov -72(%rbp), %r12
 	mov -80(%rbp), %r13
 	mov -88(%rbp), %r14
 	mov -96(%rbp), %r15
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov $0, %r12 
	mov %rsi, %r13 
	mov %rdi, %r14 
	mov $0, %r15 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov -104(%rbp), %r12
 	mov -112(%rbp), %r13
 	mov -120(%rbp), %r14
 	mov -128(%rbp), %r15
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov $0, %r12 
	mov %rsi, %r13 
	mov %rdi, %r14 
	mov $0, %r15 
	mov $1, %rsi
_uuid_22:
	cmp $0, %rsi
	jne _uuid_31
	mov $0, %rsi
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	cmp $0, %r13
	je _uuid_23
	mov 0(%r13), %r12 
	mov 16(%r13), %r14 
	mov 24(%r13), %r15 
	mov 8(%r13), %r13 
	mov %r12, -136(%rbp)
 	mov %r13, -144(%rbp)
 	mov %r14, -152(%rbp)
 	mov %r15, -160(%rbp)
	mov $1, %rsi
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	cmp $0, %rsi
	je _uuid_23
	mov $0, %rsi
	cmp $0, %r14
	je _uuid_23
	mov 0(%r14), %r12 
	mov 24(%r14), %r15 
	mov 8(%r14), %r13 
	mov 16(%r14), %r14 
	mov %r12, -168(%rbp)
 	mov %r13, -176(%rbp)
 	mov %r14, -184(%rbp)
 	mov %r15, -192(%rbp)
	mov $1, %rsi
_uuid_23:
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	cmp $0, %rsi
	je _uuid_31
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov -40(%rbp), %r12
 	mov -48(%rbp), %r13
 	mov -56(%rbp), %r14
 	mov -64(%rbp), %r15
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov -136(%rbp), %r12
 	mov -144(%rbp), %r13
 	mov -152(%rbp), %r14
 	mov -160(%rbp), %r15
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov $0, %r12 
	mov %rsi, %r13 
	mov %rdi, %r14 
	mov $0, %r15 
	call ctx_eval_soft 
	mov %r12, -200(%rbp)
 	mov %r13, -208(%rbp)
 	mov %r14, -216(%rbp)
 	mov %r15, -224(%rbp)
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov $0, %r12 
	mov %rsi, %r13 
	mov %rdi, %r14 
	mov $0, %r15 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov $0, %r12 
	mov %rsi, %r13 
	mov %rdi, %r14 
	mov $0, %r15 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov -40(%rbp), %r12
 	mov -48(%rbp), %r13
 	mov -56(%rbp), %r14
 	mov -64(%rbp), %r15
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov -168(%rbp), %r12
 	mov -176(%rbp), %r13
 	mov -184(%rbp), %r14
 	mov -192(%rbp), %r15
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov $0, %r12 
	mov %rsi, %r13 
	mov %rdi, %r14 
	mov $0, %r15 
	call ctx_eval_soft 
	mov %r12, -232(%rbp)
 	mov %r13, -240(%rbp)
 	mov %r14, -248(%rbp)
 	mov %r15, -256(%rbp)
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov $0, %r12 
	mov %rsi, %r13 
	mov %rdi, %r14 
	mov $0, %r15 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov -200(%rbp), %r12
 	mov -208(%rbp), %r13
 	mov -216(%rbp), %r14
 	mov -224(%rbp), %r15
	mov $0, %rsi
	cmp $0, %rsi
	jne _uuid_28
	mov $0, %rsi
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	cmp $0, %r13
	je _uuid_24
	mov 0(%r13), %r12 
	mov 16(%r13), %r14 
	mov 24(%r13), %r15 
	mov 8(%r13), %r13 
	mov $0, %rsi
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	cmp $0, %r13
	je _uuid_25
	mov 0(%r13), %r12 
	mov 16(%r13), %r14 
	mov 24(%r13), %r15 
	mov 8(%r13), %r13 
	mov %r12, %r8 
	mov %r13, %r9 
	mov %r14, %r10 
	mov %r15, %r11 
	mov %r12, %rax
	mov $ _uuid_27 , %r12 
	mov $0, %r13 
	mov $0, %r14 
	mov $0, %r15 
	mov %r12, %rbx
	call _streq
	mov %r12, %rdi
	mov %r8, %r12 
	mov %r9, %r13 
	mov %r10, %r14 
	mov %r11, %r15 
	cmp $0, %rdi
	je _uuid_26
	mov $1, %rsi
_uuid_26:
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	cmp $0, %rsi
	je _uuid_25
	mov $0, %rsi
	cmp $0, %r14
	je _uuid_25
	mov 0(%r14), %r12 
	mov 24(%r14), %r15 
	mov 8(%r14), %r13 
	mov 16(%r14), %r14 
	mov %r12, -264(%rbp)
 	mov %r13, -272(%rbp)
 	mov %r14, -280(%rbp)
 	mov %r15, -288(%rbp)
	mov $1, %rsi
_uuid_25:
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	cmp $0, %rsi
	je _uuid_24
	mov $0, %rsi
	cmp $0, %r14
	je _uuid_24
	mov 0(%r14), %r12 
	mov 24(%r14), %r15 
	mov 8(%r14), %r13 
	mov 16(%r14), %r14 
	mov %r12, -296(%rbp)
 	mov %r13, -304(%rbp)
 	mov %r14, -312(%rbp)
 	mov %r15, -320(%rbp)
	mov $1, %rsi
_uuid_24:
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	cmp $0, %rsi
	je _uuid_28
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov -40(%rbp), %r12
 	mov -48(%rbp), %r13
 	mov -56(%rbp), %r14
 	mov -64(%rbp), %r15
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov -264(%rbp), %r12
 	mov -272(%rbp), %r13
 	mov -280(%rbp), %r14
 	mov -288(%rbp), %r15
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov $0, %r12 
	mov %rsi, %r13 
	mov %rdi, %r14 
	mov $0, %r15 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov -232(%rbp), %r12
 	mov -240(%rbp), %r13
 	mov -248(%rbp), %r14
 	mov -256(%rbp), %r15
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov $0, %r12 
	mov %rsi, %r13 
	mov %rdi, %r14 
	mov $0, %r15 
	call destructure 
	mov %r12, -328(%rbp)
 	mov %r13, -336(%rbp)
 	mov %r14, -344(%rbp)
 	mov %r15, -352(%rbp)
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov $0, %r12 
	mov %rsi, %r13 
	mov %rdi, %r14 
	mov $0, %r15 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov -328(%rbp), %r12
 	mov -336(%rbp), %r13
 	mov -344(%rbp), %r14
 	mov -352(%rbp), %r15
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov -296(%rbp), %r12
 	mov -304(%rbp), %r13
 	mov -312(%rbp), %r14
 	mov -320(%rbp), %r15
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov $0, %r12 
	mov %rsi, %r13 
	mov %rdi, %r14 
	mov $0, %r15 
	call ctx_eval_soft 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov $0, %r12 
	mov %rsi, %r13 
	mov %rdi, %r14 
	mov $0, %r15 
	call tail 
	mov $1, %rsi
_uuid_28:
	cmp $0, %rsi
	jne _uuid_29
	mov $1, %rsi
	cmp $0, %rsi
	je _uuid_29
	mov -200(%rbp), %r12
 	mov -208(%rbp), %r13
 	mov -216(%rbp), %r14
 	mov -224(%rbp), %r15
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov -232(%rbp), %r12
 	mov -240(%rbp), %r13
 	mov -248(%rbp), %r14
 	mov -256(%rbp), %r15
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov $0, %r12 
	mov %rsi, %r13 
	mov %rdi, %r14 
	mov $0, %r15 
	mov $1, %rsi
_uuid_29:
	cmp $0, %rsi
	jne _uuid_30
	mov $0, %r12 
	mov $0, %r13 
	mov $0, %r14 
	mov $0, %r15 
_uuid_30:
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov $0, %r12 
	mov %rsi, %r13 
	mov %rdi, %r14 
	mov $0, %r15 
	call tail 
	mov $1, %rsi
_uuid_31:
	cmp $0, %rsi
	jne _uuid_32
	mov %r12, -360(%rbp)
 	mov %r13, -368(%rbp)
 	mov %r14, -376(%rbp)
 	mov %r15, -384(%rbp)
	mov $1, %rsi
	cmp $0, %rsi
	je _uuid_32
	mov -40(%rbp), %r12
 	mov -48(%rbp), %r13
 	mov -56(%rbp), %r14
 	mov -64(%rbp), %r15
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov -360(%rbp), %r12
 	mov -368(%rbp), %r13
 	mov -376(%rbp), %r14
 	mov -384(%rbp), %r15
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov $0, %r12 
	mov %rsi, %r13 
	mov %rdi, %r14 
	mov $0, %r15 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov -360(%rbp), %r12
 	mov -368(%rbp), %r13
 	mov -376(%rbp), %r14
 	mov -384(%rbp), %r15
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov $0, %r12 
	mov %rsi, %r13 
	mov %rdi, %r14 
	mov $0, %r15 
	call kv_lookup 
	mov $1, %rsi
_uuid_32:
	cmp $0, %rsi
	jne _uuid_33
	mov $0, %r12 
	mov $0, %r13 
	mov $0, %r14 
	mov $0, %r15 
_uuid_33:
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	mov %rbp, %rsp 
	pop %rbp 
	ret 
eval_soft:
	push %rbp 
	mov %rsp, %rbp 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	mov %r12, -8(%rbp)
 	mov %r13, -16(%rbp)
 	mov %r14, -24(%rbp)
 	mov %r15, -32(%rbp)
	mov $ _uuid_34 , %r12 
	mov $0, %r13 
	mov $0, %r14 
	mov $0, %r15 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov -8(%rbp), %r12
 	mov -16(%rbp), %r13
 	mov -24(%rbp), %r14
 	mov -32(%rbp), %r15
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov $0, %r12 
	mov %rsi, %r13 
	mov %rdi, %r14 
	mov $0, %r15 
	call ctx_eval_soft 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	mov %rbp, %rsp 
	pop %rbp 
	ret 
kv_lookup:
	push %rbp 
	mov %rsp, %rbp 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	mov %r12, %r8 
	mov %r13, %r9 
	mov %r14, %r10 
	mov %r15, %r11 
	call tail 
	mov %r12, -8(%rbp)
 	mov %r13, -16(%rbp)
 	mov %r14, -24(%rbp)
 	mov %r15, -32(%rbp)
	mov %r8, %r12 
	mov %r9, %r13 
	mov %r10, %r14 
	mov %r11, %r15 
	call head 
	mov %r12, %r8 
	mov %r13, %r9 
	mov %r14, %r10 
	mov %r15, %r11 
	call tail 
	mov %r12, -40(%rbp)
 	mov %r13, -48(%rbp)
 	mov %r14, -56(%rbp)
 	mov %r15, -64(%rbp)
	mov %r8, %r12 
	mov %r9, %r13 
	mov %r10, %r14 
	mov %r11, %r15 
	call head 
	mov %r12, -72(%rbp)
 	mov %r13, -80(%rbp)
 	mov %r14, -88(%rbp)
 	mov %r15, -96(%rbp)
	mov -72(%rbp), %r12
 	mov -80(%rbp), %r13
 	mov -88(%rbp), %r14
 	mov -96(%rbp), %r15
	mov $0, %rsi
	cmp $0, %rsi
	jne _uuid_39
	mov $0, %rsi
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	cmp $0, %r13
	je _uuid_35
	mov 0(%r13), %r12 
	mov 16(%r13), %r14 
	mov 24(%r13), %r15 
	mov 8(%r13), %r13 
	mov %r12, -104(%rbp)
 	mov %r13, -112(%rbp)
 	mov %r14, -120(%rbp)
 	mov %r15, -128(%rbp)
	mov $1, %rsi
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	cmp $0, %rsi
	je _uuid_35
	mov $0, %rsi
	cmp $0, %r14
	je _uuid_35
	mov 0(%r14), %r12 
	mov 24(%r14), %r15 
	mov 8(%r14), %r13 
	mov 16(%r14), %r14 
	mov $0, %rsi
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	cmp $0, %r13
	je _uuid_36
	mov 0(%r13), %r12 
	mov 16(%r13), %r14 
	mov 24(%r13), %r15 
	mov 8(%r13), %r13 
	mov %r12, -136(%rbp)
 	mov %r13, -144(%rbp)
 	mov %r14, -152(%rbp)
 	mov %r15, -160(%rbp)
	mov $1, %rsi
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	cmp $0, %rsi
	je _uuid_36
	mov $0, %rsi
	cmp $0, %r14
	je _uuid_36
	mov 0(%r14), %r12 
	mov 24(%r14), %r15 
	mov 8(%r14), %r13 
	mov 16(%r14), %r14 
	mov %r12, -168(%rbp)
 	mov %r13, -176(%rbp)
 	mov %r14, -184(%rbp)
 	mov %r15, -192(%rbp)
	mov $1, %rsi
_uuid_36:
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
_uuid_35:
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	cmp $0, %rsi
	je _uuid_39
	mov -136(%rbp), %r12
 	mov -144(%rbp), %r13
 	mov -152(%rbp), %r14
 	mov -160(%rbp), %r15
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov -40(%rbp), %r12
 	mov -48(%rbp), %r13
 	mov -56(%rbp), %r14
 	mov -64(%rbp), %r15
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov $0, %r12 
	mov %rsi, %r13 
	mov %rdi, %r14 
	mov $0, %r15 
	call eq 
	cmp $0, %r12
	jne _uuid_37
	cmp $0, %r13
	jne _uuid_37
	cmp $0, %r14
	jne _uuid_37
	cmp $0, %r15
	jne _uuid_37
	mov -104(%rbp), %r12
 	mov -112(%rbp), %r13
 	mov -120(%rbp), %r14
 	mov -128(%rbp), %r15
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov -40(%rbp), %r12
 	mov -48(%rbp), %r13
 	mov -56(%rbp), %r14
 	mov -64(%rbp), %r15
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov $0, %r12 
	mov %rsi, %r13 
	mov %rdi, %r14 
	mov $0, %r15 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov -8(%rbp), %r12
 	mov -16(%rbp), %r13
 	mov -24(%rbp), %r14
 	mov -32(%rbp), %r15
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov $0, %r12 
	mov %rsi, %r13 
	mov %rdi, %r14 
	mov $0, %r15 
	call kv_lookup 
	jmp _uuid_38
_uuid_37:
	mov -168(%rbp), %r12
 	mov -176(%rbp), %r13
 	mov -184(%rbp), %r14
 	mov -192(%rbp), %r15
_uuid_38:
	mov $1, %rsi
_uuid_39:
	cmp $0, %rsi
	jne _uuid_40
	mov $1, %rsi
	cmp $0, %rsi
	je _uuid_40
	mov -8(%rbp), %r12
 	mov -16(%rbp), %r13
 	mov -24(%rbp), %r14
 	mov -32(%rbp), %r15
	mov $1, %rsi
_uuid_40:
	cmp $0, %rsi
	jne _uuid_41
	mov $0, %r12 
	mov $0, %r13 
	mov $0, %r14 
	mov $0, %r15 
_uuid_41:
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	mov %rbp, %rsp 
	pop %rbp 
	ret 
concat:
	push %rbp 
	mov %rsp, %rbp 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	pushq $0 
	mov %r12, %r8 
	mov %r13, %r9 
	mov %r14, %r10 
	mov %r15, %r11 
	call tail 
	mov %r12, -8(%rbp)
 	mov %r13, -16(%rbp)
 	mov %r14, -24(%rbp)
 	mov %r15, -32(%rbp)
	mov %r8, %r12 
	mov %r9, %r13 
	mov %r10, %r14 
	mov %r11, %r15 
	call head 
	mov %r12, -40(%rbp)
 	mov %r13, -48(%rbp)
 	mov %r14, -56(%rbp)
 	mov %r15, -64(%rbp)
	mov -40(%rbp), %r12
 	mov -48(%rbp), %r13
 	mov -56(%rbp), %r14
 	mov -64(%rbp), %r15
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov -8(%rbp), %r12
 	mov -16(%rbp), %r13
 	mov -24(%rbp), %r14
 	mov -32(%rbp), %r15
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov $0, %r12 
	mov %rsi, %r13 
	mov %rdi, %r14 
	mov $0, %r15 
	mov $0, %rsi
	cmp $0, %rsi
	jne _uuid_44
	mov $0, %rsi
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	cmp $0, %r13
	je _uuid_42
	mov 0(%r13), %r12 
	mov 16(%r13), %r14 
	mov 24(%r13), %r15 
	mov 8(%r13), %r13 
	mov %r12, -72(%rbp)
 	mov %r13, -80(%rbp)
 	mov %r14, -88(%rbp)
 	mov %r15, -96(%rbp)
	mov $1, %rsi
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	cmp $0, %rsi
	je _uuid_42
	mov $0, %rsi
	cmp $0, %r14
	je _uuid_42
	mov 0(%r14), %r12 
	mov 24(%r14), %r15 
	mov 8(%r14), %r13 
	mov 16(%r14), %r14 
	cmp $0, %r12
	jne _uuid_43
	cmp $0, %r13
	jne _uuid_43
	cmp $0, %r14
	jne _uuid_43
	cmp $0, %r15
	jne _uuid_43
	mov $1, %rsi
_uuid_43:
_uuid_42:
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	cmp $0, %rsi
	je _uuid_44
	mov -72(%rbp), %r12
 	mov -80(%rbp), %r13
 	mov -88(%rbp), %r14
 	mov -96(%rbp), %r15
	mov $1, %rsi
_uuid_44:
	cmp $0, %rsi
	jne _uuid_47
	mov $0, %rsi
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	cmp $0, %r13
	je _uuid_45
	mov 0(%r13), %r12 
	mov 16(%r13), %r14 
	mov 24(%r13), %r15 
	mov 8(%r13), %r13 
	mov %r12, -104(%rbp)
 	mov %r13, -112(%rbp)
 	mov %r14, -120(%rbp)
 	mov %r15, -128(%rbp)
	mov $1, %rsi
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	cmp $0, %rsi
	je _uuid_45
	mov $0, %rsi
	cmp $0, %r14
	je _uuid_45
	mov 0(%r14), %r12 
	mov 24(%r14), %r15 
	mov 8(%r14), %r13 
	mov 16(%r14), %r14 
	mov $0, %rsi
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	cmp $0, %r13
	je _uuid_46
	mov 0(%r13), %r12 
	mov 16(%r13), %r14 
	mov 24(%r13), %r15 
	mov 8(%r13), %r13 
	mov %r12, -136(%rbp)
 	mov %r13, -144(%rbp)
 	mov %r14, -152(%rbp)
 	mov %r15, -160(%rbp)
	mov $1, %rsi
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	cmp $0, %rsi
	je _uuid_46
	mov $0, %rsi
	cmp $0, %r14
	je _uuid_46
	mov 0(%r14), %r12 
	mov 24(%r14), %r15 
	mov 8(%r14), %r13 
	mov 16(%r14), %r14 
	mov %r12, -168(%rbp)
 	mov %r13, -176(%rbp)
 	mov %r14, -184(%rbp)
 	mov %r15, -192(%rbp)
	mov $1, %rsi
_uuid_46:
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
_uuid_45:
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	cmp $0, %rsi
	je _uuid_47
	mov -104(%rbp), %r12
 	mov -112(%rbp), %r13
 	mov -120(%rbp), %r14
 	mov -128(%rbp), %r15
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov -136(%rbp), %r12
 	mov -144(%rbp), %r13
 	mov -152(%rbp), %r14
 	mov -160(%rbp), %r15
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov $0, %r12 
	mov %rsi, %r13 
	mov %rdi, %r14 
	mov $0, %r15 
	call concat 
	push %r12 
	push %r13 
	push %r14 
	push %r15 
	mov -168(%rbp), %r12
 	mov -176(%rbp), %r13
 	mov -184(%rbp), %r14
 	mov -192(%rbp), %r15
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	pop %r15 
	pop %r14 
	pop %r13 
	pop %r12 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov $0, %r12 
	mov %rsi, %r13 
	mov %rdi, %r14 
	mov $0, %r15 
	mov $1, %rsi
_uuid_47:
	cmp $0, %rsi
	jne _uuid_48
	mov $0, %r12 
	mov $0, %r13 
	mov $0, %r14 
	mov $0, %r15 
_uuid_48:
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	pop %r8 
	mov %rbp, %rsp 
	pop %rbp 
	ret 
main:
	pop %rax 
	mov $__argv, %rbx 
	movq $0, 0(%rbx) 
	movq $0, 8(%rbx) 
	movq $0, 16(%rbx) 
	movq $0, 24(%rbx) 
__before_main_argv: 
	cmp $0, %rax 
	je __before_main_end 
	pop %r12 
	mov $0, %r13 
	mov $0, %r14 
	mov $0, %r15 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rsi, %rdi 
	mov $0, %r12 
	mov $0, %r13 
	mov $0, %r14 
	mov $0, %r15 
	mov $__s_section, %rsi 
	mov $__s_counter, %r8 
	mov 0(%r8), %r10 
	add %r10, %rsi 
	add $32, %r10 
	mov %r10, 0(%r8) 
	mov %r12, 0(%rsi) 
	mov %r13, 8(%rsi) 
	mov %r14, 16(%rsi) 
	mov %r15, 24(%rsi) 
	mov %rdi, 8(%rbx) 
	mov %rsi, 16(%rbx) 
	mov %rsi, %rbx 
	dec %rax 
	jmp __before_main_argv 
__before_main_end: 
	push %rbp 
	mov %rsp, %rbp 
	mov $ _uuid_49 , %r12 
	mov $0, %r13 
	mov $0, %r14 
	mov $0, %r15 
	call print_s 

	mov $60, %eax 
	mov $0, %edi 
	syscall 

.data 

load_file_bsz: 
	.quad 1024 
load_file_buf: 
	.zero 1024 
__nil: 
	.zero 32 
__argv: 
	.zero 32 
__s_counter: 
	.zero 8 
__s_section: 
	.zero 131072 
__a_counter: 
	.zero 8 
__a_section: 
	.zero 131072 
__nil_literal: 
	.ascii "()" 
	.zero 1 
__hex_buffer: 
	.ascii "0123456789abcdef" 
__put64_buffer: 
	.ascii "0x" 
__put64_write_buffer: 
	.ascii "0000000000000000 " 
	.zero 1 
__newline: 
	.ascii "\n" 
__lparen: 
	.ascii "(" 
__rparen: 
	.ascii ")" 
__space: 
	.ascii " " 
__true: 
	.ascii "True" 
	.zero 1 
__err_fopen: 
	.ascii "Could not open file." 
	.zero 1 
_uuid_4:
	.ascii "Var"
	.zero 1
_uuid_20:
	.ascii "Lambda"
	.zero 1
_uuid_21:
	.ascii "Lambda"
	.zero 1
_uuid_27:
	.ascii "Lambda"
	.zero 1
_uuid_34:
	.ascii "CTX"
	.zero 1
_uuid_49:
	.ascii "hello_world"
	.zero 1
