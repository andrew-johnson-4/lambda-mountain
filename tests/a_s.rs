
use lambda_mountain::*;

#[test]
fn s_equality() {
   assert_eq!( s_nil().to_string(), "()" );
   assert_eq!( s_atom("a").to_string(), "a" );
   assert_eq!( s_cons(s_atom("a"),s_nil()).to_string(), "(a . ())" );
   assert_eq!( head(s_cons(s_atom("a"),s_nil())).to_string(), "a" );
   assert_eq!( tail(s_cons(s_atom("a"),s_nil())).to_string(), "()" );
   assert_eq!( s_nil(), s_nil() );
   assert_eq!( s_atom("a"), s_atom("a") );
   assert_ne!( s_atom("a"), s_atom("b") );
   assert_eq!( s_cons(s_atom("a"),s_nil()), s_cons(s_atom("a"),s_nil()) );
   assert_ne!( s_cons(s_atom("a"),s_nil()), s_cons(s_nil(),s_atom("a")) );
   assert_eq!( s_cons(s_cons(s_nil(),s_atom("a")),s_nil()),  s_cons(s_cons(s_nil(),s_atom("a")),s_nil()) );
   assert_ne!( s_cons(s_cons(s_nil(),s_atom("a")),s_nil()),  s_cons(s_cons(s_nil(),s_atom("b")),s_nil()) );
}
