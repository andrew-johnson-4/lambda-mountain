
use std::borrow::Borrow;
use std::rc::Rc;

#[derive(PartialEq,Eq)]
pub enum S {
   Nil,
   Atom(String),
   Cons(Rc<S>,Rc<S>),
}


impl std::fmt::Display for S {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
         S::Nil => write!(f, "()"),
         S::Atom(s) => write!(f, "{}", s),
         S::Cons(h,t) => write!(f, "({} . {})", h, t),
      }
   }
}

impl std::fmt::Debug for S {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub fn nil() -> Rc<S> {
   Rc::new(S::Nil)
}

pub fn atom(s: &str) -> Rc<S> {
   Rc::new(S::Atom(s.to_string()))
}

pub fn cons( x: Rc<S>, y: Rc<S> ) -> Rc<S> {
   Rc::new(S::Cons( x, y ))
}

pub fn head( s: Rc<S> ) -> Rc<S> {
   if let S::Cons(h,_) = s.borrow() { h.clone() }
   else { nil() }
}

pub fn tail( s: Rc<S> ) -> Rc<S> {
   if let S::Cons(_,t) = s.borrow() { t.clone() }
   else { nil() }
}

