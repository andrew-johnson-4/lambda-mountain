use lambda_mountain::{Policy,Rhs};

fn my_print(args: &[Rhs]) -> Rhs {
   let mut s = String::new();
   for a in args {
      s.push_str(&a.to_string());
   }
   Rhs::Literal(s)
}

#[test]
fn extern0() {
   let mut p = Policy::new();
   p.bind_extern("my_print", &my_print);

   assert_eq!( p.s_hard("my_print 2"), "2" );
}
