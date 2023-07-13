use crate::ast::*;

use std::rc::Rc;
use std::collections::HashMap;
use std::collections::LinkedList;

pub struct Context {
   globals: Rc<HashMap<String,Vec<Rhs>>>,
   locals: LinkedList<(String,Rhs)>,
}

impl Context {
   pub fn new(policy: Rc<HashMap<String,Vec<Rhs>>>) -> Context {
      Context {
         globals: policy,
         locals: LinkedList::new(),
      }
   }
}

pub fn eval_parse(context: Context, rule: &str, input: StringSlice) -> String {
   for rhs in context.globals.get(rule).expect(rule) {
      unimplemented!("apply parse rule: {}", rhs)
   }
   panic!("Parse Error [{}]: {}", rule, input.to_string())
}

/*
pub fn eval_rhs(context: LinkedList<String,Rhs>, term: Rhs) -> Rhs {
   if let Rhs::App(gs) = term {
      let gs = gs.iter().map(|g| eval(context, g)).collect();
      if let Rhs::Lambda(lhs,rhs) = gs[0] {
         let inner_context = destructure(context, lhs, gs[1..]);
         eval(inner_context, rhs);
      } else {
         Rhs::App(gs)
      }
   } else if let Rhs::Variable(v) = term {
      context.lookup(v)
   } else {
      term
   }
}

pub fn eval_destructure(mut context: LinkedList<String,Rhs>, lhs: Vec<Lhs>, rhs: Vec<Rhs>)
   -> LinkedList<String,Rhs> {
   if lhs.len() != rhs.len() {
      panic!("Wrong Arity")
   }
   for (l,r) in std::iter::zip(lhs,rhs) {
      if let (Lhs::App(ls),Rhs::App(rs)) = (l,r) {
         context = destructure(context,ls,rs);
      } else if let (Lhs::Variable(lv),rv) = (l,r) {
         context = context.bind(lv, rv);
      }
   }
   context
}
*/
