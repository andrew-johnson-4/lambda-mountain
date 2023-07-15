use lambda_mountain::Policy;

#[test]
fn infer_ascript0() {
   let mut p = Policy::new();

   assert_eq!( p.s_infer("(: x Int)"), "(: x Int)" );
   assert_eq!( p.s_infer("(: x Float)"), "(: x Float)" );
}

#[test]
fn infer_ascript1() {
   let mut p = Policy::new();
   p.s_load("x := (: 123 Int)");

   assert_eq!( p.s_infer("x"), "(: x Int)" );
}

#[test]
fn infer_literal0() {
   let mut p = Policy::new();

   assert_eq!( p.s_infer("1"), "(: 1 Int)" );
   assert_eq!( p.s_infer("1.2"), "(: 1 Float)" );
}

#[test]
fn infer_abs0() {
   let mut p = Policy::new();

   assert_eq!( p.s_infer("λ(: s Int). s"), "(: (λ(: s Int). (: s Int)) (Arrow Int Int))");
}

#[test]
fn infer_app0() {
   let mut p = Policy::new();
   p.s_load("p := λ(: s Int). s");

   assert_eq!( p.s_infer("p 1"), "(: ((: p (Arrow Int Int)) (: 1 Int)) Int)");
   assert!( p.infer("p 1.2").is_err() );
}

