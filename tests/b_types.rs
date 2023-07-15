use lambda_mountain::Policy;

#[test]
fn infer_ascript0() {
   let mut p = Policy::new();
   p.f_load("preludes/simply_typed.lm");

   assert_eq!( p.s_soft("(: x Int)"), "(: x Int)" );
   assert_eq!( p.s_soft("(: x Float)"), "(: x Float)" );
}

#[test]
fn infer_literal0() {
   let mut p = Policy::new();
   p.f_load("preludes/simply_typed.lm");

   assert_eq!( p.s_soft("1"), "(: 1 Int)" );
   assert_eq!( p.s_soft("1.2"), "(: 1.2 Float)" );
}

#[test]
fn infer_abs0() {
   let mut p = Policy::new();
   p.f_load("preludes/simply_typed.lm");

   assert_eq!( p.s_soft("λ(: s Int). s"), "(: (λ(: s Int). (: s Int)) (Arrow Int Int))");
}

#[test]
fn infer_app0() {
   let mut p = Policy::new();
   p.f_load("preludes/simply_typed.lm");
   p.s_load("p := λ(: s Int). s");

   assert_eq!( p.s_soft("p 1"), "(: ((: p (Arrow Int Int)) (: 1 Int)) Int)");
   assert!( p.soft("p 1.2").is_err() );
}

