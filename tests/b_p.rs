
use lambda_mountain::*;

#[test]
fn expressions() {
   assert_eq!( parse_expression("a").to_string(), "(Variable a)" );
   assert_eq!( parse_expression("'a").to_string(), "(Literal a)" );
   assert_eq!( parse_expression("Lambda").to_string(), "(Literal Lambda)" );
   assert_eq!( parse_expression("a b").to_string(), "(App ((Variable a) (Variable b)))" );
   assert_eq!( parse_expression("a b c").to_string(), "(App ((App ((Variable a) (Variable b))) (Variable c)))" );
   assert_eq!( parse_expression("123").to_string(), "(Literal 123)" );
   assert_eq!( parse_expression("λx.y").to_string(), "(Lambda ((Variable x) (Variable y)))" );
   assert_eq!( parse_expression("λx.y z").to_string(), "(Lambda ((Variable x) (App ((Variable y) (Variable z)))))" );
   assert_eq!( parse_expression("(λx.y) z").to_string(), "(App ((Lambda ((Variable x) (Variable y))) (Variable z)))" );
   assert_eq!( parse_expression("λx y.z").to_string(), "(Lambda ((App ((Variable x) (Variable y))) (Variable z)))" );
   assert_eq!( parse_expression("λx y z.z").to_string(), "(Lambda ((App ((App ((Variable x) (Variable y))) (Variable z))) (Variable z)))" );
}

#[test]
fn programs() {
   assert_eq!( parse_program("a := b").to_string(), "(() (Global (a (Variable b))))" );
}
