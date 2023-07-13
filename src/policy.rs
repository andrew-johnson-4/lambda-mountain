use crate::ast::*;
use crate::parser::*;

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
      let input = StringSlice::new(input.to_string() + "\n");
      for (symbol,rhs) in parse_program(input) {
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
      unimplemented!("Policy::soft");
      /*
      let input = if self.symbols.contains_key("::pre") {
         self.parse(&vec![], "::pre", input).to_string()
      } else {
         input.to_string()
      };
      input
      */
   }
}

