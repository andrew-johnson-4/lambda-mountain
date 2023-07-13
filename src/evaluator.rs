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
   pub fn bind(self, symbol: String, term: Rhs) -> Context {
      let mut locals = self.locals.clone();
      locals.push_front((symbol, term));
      Context {
         globals: self.globals.clone(),
         locals: locals,
         is_null: false,
      }
   }
   pub fn lookup(self, symbol: &str) -> Rhs {
      for (k,v) in self.locals {
         if k==symbol {
            return v.clone();
         }
      }
      if let Some(vs) = self.globals.get(symbol) {
         return Rhs::Poly(vs.clone());
      }
      Rhs::Variable(symbol.to_string())
   }
}

pub fn eval_parse(context: Context, rule: &str, input: StringSlice) -> String {
   for rhs in context.globals.get(rule).expect(rule) {
      if let Rhs::Lambda(lhs,rhs) = rhs {
         let context = destructure_literal(context.clone(), lhs, input.clone());
         if !context.is_null {
            let rhs = eval_rhs(context.clone(), rhs);
            let rhs = if let Rhs::App(rs) = rhs {
               let mut s = String::new();
               for rv in rs {
                  if let Rhs::Literal(l) = rv {
                     s.push_str(&l);
                  } else {
                     s.push_str(&rv.to_string());
                  }
               }
               Rhs::Literal(s)
            } else { rhs };
            return rhs.to_string();
         }
      } else {
         return rhs.to_string();
      }
   }
   panic!("Parse Error [{}]: {}", rule, input.to_string())
}

pub fn destructure_literal(context: Context, lhs: &[Lhs], input: StringSlice) -> Context {
   if lhs.len()==0 {
      if input.len()==0 {
         context
      } else {
         Context::null()
      }
   } else if let Lhs::Literal(v) = &lhs[0] {
      if input.starts_with(v) {
         destructure_literal( context, &lhs[1..], input.after(v.len()) )
      } else {
         Context::null()
      }
   } else if let Lhs::Literal(v) = &lhs[lhs.len()-1] {
      if input.ends_with(v) {
         destructure_literal( context, &lhs[..lhs.len()-1], input.before(v.len()) )
      } else {
         Context::null()
      }
   } else if let Lhs::Variable(v) = &lhs[0] {
      context.bind(v.clone(), Rhs::Literal(input.to_string()) )
   } else {
      unimplemented!("evaluator::destructure_literal {}", lhs[0])
   }
}

pub fn eval_rhs(context: Context, rhs: &[Rhs]) -> Rhs {
   if rhs.len()==0 {
      Rhs::App(Vec::new())
   } else if rhs.len()==1 {
      if let Rhs::Variable(v) = &rhs[0] {
         context.lookup(v)
      } else if let Rhs::App(gs) = &rhs[0] {
         eval_rhs(context, gs)
      } else {
         rhs[0].clone()
      }
   } else {
      let gs = rhs.iter().map(|g| eval_rhs(context.clone(), &[g.clone()])).collect::<Vec<Rhs>>();
      if let Rhs::Lambda(lhs,rhs) = &gs[0] {
         let inner_context = destructure_rhs(context.clone(), lhs, &gs[1..]);
         eval_rhs(inner_context, rhs)
      } else {
         Rhs::App(gs)
      }
   }
}

pub fn destructure_rhs(mut context: Context, lhs: &[Lhs], rhs: &[Rhs]) -> Context {
   unimplemented!("evaluator::destructure_rhs")
/*
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
}

