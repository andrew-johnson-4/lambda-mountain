
punc!(
   //strlen(%rdx: String) -> %rax
   (label strlen)
   (xor %rax %rax)
   (jmp strlen_overit)
   (label strlen_looptop)
   (inc %rdx)
   (inc %rax)
   (label strlen_overit)
   (mov @0 %bx)
   (cmp [%rdx] %bx)
   (jnz strlen_looptop)
   (ret)

   //print(%rsi %rdi: S)
   (label print)
   (label print_atom)

   //if atom (%rsi is not zero)
   (cmp @0 %rsi)
   (je print_cons)
   (mov %rsi %rdx)
   (call strlen)
   (mov %rax %rdx) //set write length to strlen
   //%rsi is pointer to string
   (mov @1 %rax) //set write as command
   (mov @1 %rdi) //set output to STDOUT
   (syscall)
   (jmp print_ret)

   //if cons (%rdi is not zero)
   (label print_cons)
   (cmp @0 %rdi)
   (je print_nil)
   //TODO: print cons
   (jmp print_ret)

   //if nil (%rsi and %rdi are zero)
   (label print_nil)
   (mov @1 %rax)
   (mov @1 %rdi)
   (mov @nil_as_string %rdx)
   (call strlen)
   (mov %rax %rdx)
   (mov @nil_as_string %rsi)
   (syscall)
   (jmp print_ret)

   (label print_ret)
   (ret)
)
