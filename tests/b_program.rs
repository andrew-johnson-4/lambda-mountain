use lambda_mountain::Policy;

#[test]
fn eval_echo0() {
   let mut p = Policy::new();
   p.s_load("p := λs.");

   assert_eq!( p.s_hard("p a"), "()" );
   assert_eq!( p.s_hard("p (a a)"), "()" );
   assert_eq!( p.s_hard("p (a a a)"), "()" );
}

#[test]
fn eval_echo1() {
   let mut p = Policy::new();
   p.s_load("p := λs. s");

   assert_eq!( p.s_hard("p a"), "a" );
   assert_eq!( p.s_hard("p (a a)"), "(a a)" );
   assert_eq!( p.s_hard("p (a a a)"), "(a a a)" );
}

#[test]
fn eval_echo2() {
   let mut p = Policy::new();
   p.s_load("p := λs. s s");

   assert_eq!( p.s_hard("p a"), "(a a)" );
   assert_eq!( p.s_hard("p (a a)"), "((a a) (a a))" );
   assert_eq!( p.s_hard("p (a a a)"), "((a a a) (a a a))" );
}

#[test]
fn eval_pat1() {
   let mut p = Policy::new();
   p.s_load("p := λ[ s. s\np := λs. s");

   assert_eq!( p.s_hard("p [ a"), "a" );
   assert_eq!( p.s_hard("p a"), "a" );
}

#[test]
fn eval_pat2() {
   let mut p = Policy::new();
   p.s_load("p := λs ]. s\np := λs. s");

   assert_eq!( p.s_hard("p a ]"), "a" );
   assert_eq!( p.s_hard("p a"), "a" );
}

#[test]
fn eval_pat3() {
   let mut p = Policy::new();
   p.s_load("p := λ[ s ]. s\np := λs. s");

   assert_eq!( p.s_hard("p [ a ]"), "a" );
   assert_eq!( p.s_hard("p a"), "a" );
}
