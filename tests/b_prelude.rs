use lambda_mountain::Policy;

#[test]
fn print_let0() {
   let mut p = Policy::new();
   assert_eq!( p.s_hard("let (x 1) x"), "1" );
}

#[test]
fn print_match0() {
   let mut p = Policy::new();
   assert_eq!( p.s_hard("match 1 ((λ1. 2) (λ3. 4))"), "2" );
   assert_eq!( p.s_hard("match 3 ((λ1. 2) (λ3. 4))"), "4" );
}
