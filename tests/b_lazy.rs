use lambda_mountain::Policy;

#[test]
fn eval_f0() {
   let mut p = Policy::new();
   p.s_load("f := λx.x");

   assert_eq!( p.s_hard("(T (f 1))"), "(T 1)" );
}
