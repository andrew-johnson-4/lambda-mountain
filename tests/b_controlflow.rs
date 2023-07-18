use lambda_mountain::{Policy,Rhs};

fn once(args: &Rhs) -> Rhs {
   Rhs::Literal("True".to_string())
}

#[test]
fn print_while0() {
   let mut p = Policy::new();
   assert_eq!( p.s_hard("while False (λ.print Hello World)"), "" );
   assert_eq!( p.s_hard("while (once ()) (λ.print Hello World)"), "Hello World" );
}

#[test]
fn print_foreach0() {
   let mut p = Policy::new();
   assert_eq!( p.s_hard("foreach (2 3 4 5) (λx.x)"), "(2 3 4 5)");
}

#[test]
fn print_if0() {
   let mut p = Policy::new();
   assert_eq!( p.s_hard("if True 1"), "1");
   assert_eq!( p.s_hard("if True 1 2"), "1");
   assert_eq!( p.s_hard("if False 1 2"), "2");
   assert_eq!( p.s_hard("if B 1 2"), "2");
}

#[test]
fn print_match0() {
   let mut p = Policy::new();
   assert_eq!( p.s_hard("match (A 1) ( (λA a.a) (λB b.b) )"), "1");
   assert_eq!( p.s_hard("match (B 2) ( (λA a.a) (λB b.b) )"), "2");
}
