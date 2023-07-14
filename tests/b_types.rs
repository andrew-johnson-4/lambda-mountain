use lambda_mountain::Policy;

#[test]
fn eval_id0() {
   let mut p = Policy::new();
   p.s_load("p := λ(: s Int). (: s Int)");

   assert_eq!( p.s_hard("p (: 1 Int)"), "(: 1 Int)" );
   assert!( p.hard("p (: 1.2 Float)").is_err() );
}
