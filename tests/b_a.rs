
use lambda_mountain::*;

#[test]
fn equality() {
   assert_eq!( literal("a").to_string(), "(Literal a)" );
   assert_eq!( literal("bc").to_string(), "(Literal bc)" );

   assert_eq!( variable("a").to_string(), "(Variable a)" );
   assert_eq!( variable("bc").to_string(), "(Variable bc)" );

   assert_eq!( lambda(literal("a"), variable("bc")).to_string(), "(Lambda ((Literal a) (Variable bc)))");
   assert_eq!( app(literal("f"),literal("x")).to_string(), "(App ((Literal f) (Literal x)))");
   assert_eq!( list(&[literal("a"), variable("c")]).to_string(), "((Literal a) ((Variable c) ()))" );
}

#[test]
fn destructure() {
   assert_eq!( map( literal("1"), literal("1"), literal("123") ).to_string(), "(Literal 123)" );
   assert_eq!( map( literal("1"), literal("2"), literal("123") ).to_string(), "()" );
   assert_eq!( map( variable("a"), literal("123"), variable("a") ).to_string(), "(Literal 123)" );
   assert_eq!( map( variable("a"), literal("123"), variable("b") ).to_string(), "(Variable b)" );

   assert_eq!( map(
      lambda(literal("a"), variable("bc")),
      lambda(literal("a"), variable("bc")),
      literal("123"),
   ).to_string(), "(Literal 123)" );
   assert_eq!( map(
      lambda(literal("a"), variable("bc")),
      lambda(literal("a"), variable("b")),
      literal("123"),
   ).to_string(), "()" );

   assert_eq!( map(
      list(&[ literal("a"), variable("b") ]),
      list(&[ literal("a"), literal("b") ]),
      variable("b"),
   ).to_string(), "(Literal b)" );
   assert_eq!( map(
      list(&[ literal("a"), literal("b") ]),
      list(&[ literal("a"), literal("c") ]),
      variable("b"),
   ).to_string(), "()" );
}
