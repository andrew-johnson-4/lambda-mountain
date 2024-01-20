
use lambda_mountain::*;

#[test]
fn test_eval() {
   assert_eq!( eval_soft(&parse_expression("a")).to_string(), "(variable . a)" );
   assert_eq!( eval_soft(&parse_expression("123")).to_string(), "(literal . 123)" );
   assert_eq!( eval_soft(&parse_expression("λx.y")).to_string(), "(lambda . ((variable . x) . (variable . y)))" );
   assert_eq!( eval_soft(&parse_expression("x y")).to_string(), "(app . ((variable . x) . (variable . y)))" );
   assert_eq!( eval_soft(&parse_expression("(λx.x) z")).to_string(), "(variable . z)" );
}

#[test]
fn test_ctx_eval() {
   assert_eq!( ctx_eval_soft(&kv(&[(s_atom("a"),literal("b"))]),&parse_expression("b")).to_string(), "(variable . b)" );
   assert_eq!( ctx_eval_soft(&kv(&[(s_atom("a"),literal("b"))]),&parse_expression("a")).to_string(), "(literal . b)" );
   assert_eq!( ctx_eval_soft(&kv(&[(s_atom("a"),literal("b"))]),&parse_expression("(λx.a) z")).to_string(), "(literal . b)" );
}
