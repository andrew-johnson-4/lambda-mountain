
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
}

#[test]
fn datastructures() {
}
