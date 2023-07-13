use crate::ast::*;
use crate::parser::*;
use crate::evaluator::*;

use std::rc::Rc;
use std::collections::HashMap;

pub struct Policy {
   symbols: HashMap<String,Vec<Rhs>>,
}

impl Policy {
   pub fn new() -> Policy {
      Policy {
         symbols: HashMap::new()
      }
   }
   pub fn load(&mut self, input: &str) {
      let input = StringSlice::new(input.to_string());
      for (symbol,rhs) in parse_program(input) {
         if !self.symbols.contains_key(&symbol) {
            self.symbols.insert(symbol.clone(), Vec::new());
         }
         self.symbols.get_mut(&symbol).expect("Policy::load")
                     .push(rhs);
      }
   }
   pub fn hard(&mut self, input: &str) -> String {
      let context = Context::new(Rc::new(self.symbols.clone()));
      let input = StringSlice::new(input.to_string());
      let input = if self.symbols.contains_key("::pre") {
         StringSlice::new(
            eval_parse(context.clone(), "::pre", input)
         )
      } else {
         input
      };
      let program = vec![ Rhs::Variable("::program".to_string()), Rhs::Literal(input.to_string()) ];
      let post = eval_rhs(context.clone(), &program);
      post.to_string()
   }
   pub fn soft(&mut self, input: &str) -> String {
      let context = Context::new(Rc::new(self.symbols.clone()));
      let input = StringSlice::new(input.to_string());
      if self.symbols.contains_key("::pre") {
         eval_parse(context.clone(), "::pre", input)
      } else {
         input.to_string()
      }
   }
}

