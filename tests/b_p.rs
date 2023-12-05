
use lambda_mountain::*;

#[test]
fn expressions() {
   assert_eq!( parse_expression("a").to_string(), "(variable . a)" );
   assert_eq!( parse_expression("123").to_string(), "(literal . a)" );
   assert_eq!( parse_expression("/([a]+[b])[c]/").to_string(), "(regex . ([a]+[b])[c])" );
   assert_eq!( parse_expression("λx.y").to_string(), "(lambda . ((variable . x) . (variable . y)))" );
}

#[test]
fn programs() {
   assert_eq!( parse_program("a := b").to_string(), "(kv ())" );
}
