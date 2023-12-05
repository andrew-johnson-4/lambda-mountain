
use lambda_mountain::*;

#[test]
fn expressions() {
   assert_eq!( parse_expression("a").to_string(), "(variable . a)" );
   assert_eq!( parse_expression("a b").to_string(), "(app . ((variable . a) . (variable . b)))" );
   assert_eq!( parse_expression("123").to_string(), "(literal . 123)" );
   assert_eq!( parse_expression("/([a]+[b])[c]/").to_string(), "(regex . ([a]+[b])[c])" );
   assert_eq!( parse_expression("λx.y").to_string(), "(lambda . ((variable . x) . (variable . y)))" );
   assert_eq!( parse_expression("λx.y z").to_string(), "(lambda . ((variable . x) . (app . ((variable . y) . (variable . z)))))" );
}

#[test]
fn programs() {
   assert_eq!( parse_program("a := b").to_string(), "(kv . ((a  . (variable . b)) . ()))" );
}
