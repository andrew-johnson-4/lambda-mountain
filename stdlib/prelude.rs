
punc!(
   //print(%rsi %rdi: S)
   (label print)
   //(label print_atom)

   //if atom (%rsi is not zero)
   //(cmp @0 %rsi)
   //(je print_cons)
   (mov @1 %rax)
   (mov @1 %rdi)
   (mov @13 %rdx)
   (syscall)   

   //if cons (%rdi is not zero)
   //(label print_cons)
   //(cmp @0 %rdi)
   //(je print_nil)

   //if nil (%rsi and %rdi are zero)
   //(label print_nil)
   (ret)
)
