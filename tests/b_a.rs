
use lambda_mountain::*;

#[test]
fn equality() {
   assert_eq!( literal("a").to_string(), "(literal . a)" );
   assert_eq!( literal("bc").to_string(), "(literal . bc)" );

   assert_eq!( variable("a").to_string(), "(variable . a)" );
   assert_eq!( variable("bc").to_string(), "(variable . bc)" );

   assert_eq!( lambda(literal("a"), variable("bc")).to_string(), "(lambda . ((literal . a) . (variable . bc)))");
   assert_eq!( list(&[literal("a"), variable("c")]).to_string(), "((literal . a) . ((variable . c) . ()))" );
}

#[test]
fn destructure() {
   assert_eq!( map( literal("1"), literal("1"), literal("123") ).to_string(), "(literal . 123)" );
   assert_eq!( map( literal("1"), literal("2"), literal("123") ).to_string(), "()" );
   assert_eq!( map( variable("a"), literal("123"), variable("a") ).to_string(), "(literal . 123)" );
   assert_eq!( map( variable("a"), literal("123"), variable("b") ).to_string(), "(variable . b)" );

   assert_eq!( map(
      lambda(literal("a"), variable("bc")),
      lambda(literal("a"), variable("bc")),
      literal("123"),
   ).to_string(), "(literal . 123)" );
   assert_eq!( map(
      lambda(literal("a"), variable("bc")),
      lambda(literal("a"), variable("b")),
      literal("123"),
   ).to_string(), "()" );

   assert_eq!( map(
      list(&[ literal("a"), variable("b") ]),
      list(&[ literal("a"), literal("b") ]),
      variable("b"),
   ).to_string(), "(literal . b)" );
   assert_eq!( map(
      list(&[ literal("a"), literal("b") ]),
      list(&[ literal("a"), literal("c") ]),
      variable("b"),
   ).to_string(), "()" );

   assert_eq!( map(
      regex("^([abc]+)$"),
      literal("aab"),
      variable("{0}"),
   ).to_string(), "(literal . aab)");
   assert_eq!( map(
      regex("^([abc]+)$"),
      literal("abcd"),
      variable("{0}"),
   ).to_string(), "()");
}

#[test]
fn datastructures() {
   assert_eq!( map(
      kv(&[ (literal("1"),literal("23")), (list(&[literal("45")]),literal("6")) ]),
      kv(&[ (literal("1"),regex("^([0-9]+)$")) ]),
      variable("{0}"),
   ).to_string(), "(literal . 23)");
   assert_eq!( map(
      kv(&[ (literal("1"),literal("a")), (list(&[literal("45")]),literal("6")) ]),
      kv(&[ (literal("1"),regex("^([0-9]+)$")) ]),
      variable("{0}"),
   ).to_string(), "()");
}
