/*

Copyright 2023 - Andrew Johnson

This code and all related intellectual property is available under the terms of
the attached permissive MIT license. This license is intended only to protect
the future development of the project while otherwise allowing people to use
the code and IP as they would like. Please, just be nice.

A: An S-Expression based AST

*/

use crate::*;

pub fn literal(s: &str) -> S {
   s_cons( s_atom("literal"), s_atom(s) )
}

pub fn variable(s: &str) -> S {
   s_cons( s_atom("variable"), s_atom(s) )
}

pub fn lambda(l: S, r: S) -> S {
   s_cons( s_atom("lambda"), s_cons(l,r) )
}

pub fn regex(r: &str) -> S {
   s_nil()
}

pub fn list(s: &[S]) -> S {
   let mut tail = s_nil();
   for x in s.iter().rev() {
      tail = s_cons( x.clone(), tail );
   }
   tail
}

pub fn map(lhs: S, v: S, rhs: S) -> S {
   rhs
}
