
use im_lists::list::*;

#[test]
fn order() {
   let xs = List::new();
   let xs = List::cons(1, xs);
   let xs = List::cons(2, xs);
   assert_eq!( xs.iter().nth(0).unwrap(), &2 );
   assert_eq!( xs.iter().nth(1).unwrap(), &1 );

   let ys = List::cons(3, xs.clone());
   assert_eq!( ys.iter().nth(0).unwrap(), &3 );
   assert_eq!( ys.iter().nth(1).unwrap(), &2 );
   assert_eq!( ys.iter().nth(2).unwrap(), &1 );
   assert_eq!( xs.iter().nth(0).unwrap(), &2 );
   assert_eq!( xs.iter().nth(1).unwrap(), &1 );
}
