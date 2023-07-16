use lambda_mountain::Policy;

#[test]
fn print_let0() {
   let mut p = Policy::new();
   assert_eq!( p.s_hard("let x 1 x"), "1" );
}

#[test]
fn print_match0() {
   let mut p = Policy::new();
   assert_eq!( p.s_hard("match 1 ((λ1. 2) (λ3. 4))"), "2" );
   assert_eq!( p.s_hard("match 3 ((λ1. 2) (λ3. 4))"), "4" );
}

#[test]
fn print_lambda0() {
   let mut p = Policy::new();
   assert_eq!( p.s_hard("lambda w x . y z"), "(λw x.y z)" );
}

#[test]
fn print_cases0() {
   let mut p = Policy::new();
   assert_eq!( p.s_hard("cases (a b) (c d)"), "((λa.b) (λc.d))" );
}

#[test]
fn print_cons0() {
   let mut p = Policy::new();
   assert_eq!( p.s_hard("cons 1 ()"), "(1)" );
   assert_eq!( p.s_hard("cons 1 (2)"), "(1 2)" );
   assert_eq!( p.s_hard("cons 1 (2 3)"), "(1 2 3)" );
}
