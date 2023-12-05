/*

Copyright 2023 - Andrew Johnson

This code and all related intellectual property is available under the terms of
the attached permissive MIT license. This license is intended only to protect
the future development of the project while otherwise allowing people to use
the code and IP as they would like. Please, just be nice.

S: A Library for S-Expressions

This library may transparently hide certain internal data structures such as
* Regular Expressions
* Records
* Products
* Vectors
* Sets
* Maps

*/

use std::borrow::Borrow;
use std::rc::Rc;

#[derive(PartialEq,Eq)]
pub enum S_ {
   Nil,
   Atom(String),
   Cons(S,S),
}
pub type S = Rc<S_>;


impl std::fmt::Display for S_ {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
         S_::Nil => write!(f, "()"),
         S_::Atom(s) => write!(f, "{}", s),
         S_::Cons(h,t) => write!(f, "({} . {})", h, t),
      }
   }
}

impl std::fmt::Debug for S_ {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub fn s_nil() -> S {
   Rc::new(S_::Nil)
}

pub fn s_atom(s: &str) -> S {
   Rc::new(S_::Atom(s.to_string()))
}

pub fn s_cons( x: S, y: S ) -> S {
   Rc::new(S_::Cons( x, y ))
}

pub fn head( s: S ) -> S {
   if let S_::Cons(h,_) = s.borrow() { h.clone() }
   else { s_nil() }
}

pub fn tail( s: S ) -> S {
   if let S_::Cons(_,t) = s.borrow() { t.clone() }
   else { s_nil() }
}

