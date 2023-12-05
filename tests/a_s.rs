
use lambda_mountain::s::*;

#[test]
fn a() {
   assert_eq!( nil().to_string(), "()" );
   assert_eq!( atom("a").to_string(), "a" );
   assert_eq!( cons(atom("a"),nil()).to_string(), "(a . ())" );
   assert_eq!( head(cons(atom("a"),nil())).to_string(), "a" );
   assert_eq!( tail(cons(atom("a"),nil())).to_string(), "()" );
   assert_eq!( nil(), nil() );
   assert_eq!( atom("a"), atom("a") );
   assert_ne!( atom("a"), atom("b") );
   assert_eq!( cons(atom("a"),nil()), cons(atom("a"),nil()) );
   assert_ne!( cons(atom("a"),nil()), cons(nil(),atom("a")) );
   assert_eq!( cons(cons(nil(),atom("a")),nil()),  cons(cons(nil(),atom("a")),nil()) );
   assert_ne!( cons(cons(nil(),atom("a")),nil()),  cons(cons(nil(),atom("b")),nil()) );
}
