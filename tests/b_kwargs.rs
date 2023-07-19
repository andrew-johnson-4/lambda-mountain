use lambda_mountain::Policy;

#[test]
fn print_kw0() {
   let mut p = Policy::new();
   p.load(r#"f := (λ..args. 
   let (args a) = kw args (λ(A a). a)
   a
)"#).unwrap();
   p.load(r#"g := (λ..args. 
   let (args a) = kw args (λ(A a). a)
   args
)"#).unwrap();
   assert_eq!( p.s_hard("f (A 1)"), "1" );
   assert_eq!( p.s_hard("g (A 1)"), "()" );
}
