use lambda_mountain::Policy;

#[test]
fn print_echo0() {
   let mut p = Policy::new();
   p.load("::pre := λs.");

   assert_eq!( p.soft("a"), "" );
}

#[test]
fn print_echo1() {
   let mut p = Policy::new();
   p.load("::pre := λs. s");

   assert_eq!( p.soft("a"), "a" );
}

#[test]
fn print_echo2() {
   let mut p = Policy::new();
   p.load("::pre := λs. s s");

   assert_eq!( p.soft("a"), "aa" );
}
