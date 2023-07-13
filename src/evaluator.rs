use crate::ast::*;

use std::rc::Rc;
use std::collections::HashMap;
use std::collections::LinkedList;

#[derive(Clone)]
pub struct Context {
   globals: Rc<HashMap<String,Vec<Rhs>>>,
   locals: LinkedList<(String,Rhs)>,
   is_null: bool,
}

impl Context {
   pub fn new(policy: Rc<HashMap<String,Vec<Rhs>>>) -> Context {
      Context {
         globals: policy,
         locals: LinkedList::new(),
         is_null: false,
      }
   }
   pub fn null() -> Context {
      Context {
         globals: Rc::new(HashMap::new()),
         locals: LinkedList::new(),
         is_null: true,
      }
   }
}

pub fn eval_parse(context: Context, rule: &str, input: StringSlice) -> String {
   for rhs in context.globals.get(rule).expect(rule) {
      if let Rhs::Lambda(lhs,rhs) = rhs {
         let context = destructure_parse(context.clone(), lhs, input.clone());
         if !context.is_null {
            return eval_rhs(context.clone(), rhs).to_string();
         }
      } else {
         return rhs.to_string()
      }
   }
   panic!("Parse Error [{}]: {}", rule, input.to_string())
}

pub fn destructure_parse(context: Context, lhs: &Vec<Lhs>, input: StringSlice) -> Context {
   if lhs.len()==0 {
      if input.len()==0 {
         context
      } else {
         Context::null()
      }
   } else if let Lhs::Literal(v) = &lhs[0] {
      unimplemented!("evaluator::destructure_parse {}", lhs[0])     
   } else if let Lhs::Literal(v) = &lhs[lhs.len()-1] {
      unimplemented!("evaluator::destructure_parse {}", lhs[lhs.len()-1])
   } else if let Lhs::Variable(v) = &lhs[0] {
      unimplemented!("evaluator::destructure_parse {}", lhs[0])
   } else {
      unimplemented!("evaluator::destructure_parse {}", lhs[0])
   }
}

pub fn eval_rhs(context: Context, rhs: &Vec<Rhs>) -> Rhs {
   unimplemented!("evaluator::eval_rhs")
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
