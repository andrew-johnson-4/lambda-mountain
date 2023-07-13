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
         println!("Policy::load {} := {}", symbol, rhs);
         if !self.symbols.contains_key(&symbol) {
            self.symbols.insert(symbol.clone(), Vec::new());
         }
         self.symbols.get_mut(&symbol).expect("Policy::load")
                     .push(rhs);
      }
   }
   pub fn hard(&mut self, input: &str) {
      println!("Policy::hard\n{input}\n");
      unimplemented!("Policy::hard");
   }
   pub fn soft(&mut self, input: &str) -> String {
      let input = StringSlice::new(input.to_string());
      if self.symbols.contains_key("::pre") {
         eval_parse(Context::new(Rc::new(self.symbols.clone())), "::pre", input)
      } else {
         input.to_string()
      }
   }
}

