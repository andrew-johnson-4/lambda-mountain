use crate::ast::*;
use crate::parser::*;
use crate::evaluator::*;

use std::rc::Rc;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub struct Policy {
   pub symbols: HashMap<String,Vec<Rhs>>,
}

impl Policy {
   pub fn new() -> Policy {
      Policy {
         symbols: HashMap::new()
      }
   }
   pub fn bind(&mut self, k: &str, v: Rhs) {
      if !self.symbols.contains_key(k) {
         self.symbols.insert(k.to_string(), Vec::new());
      }
      self.symbols.get_mut(k).expect("Policy::load")
                  .push(v);
   }
   pub fn load(&mut self, input: &str) -> Result<(),String> {
      let input = StringSlice::new(input.to_string());
      for (symbol,rhs) in parse_program(input)? {
         self.bind(&symbol, rhs);
      }
      Result::Ok(())
   }
   pub fn s_load(&mut self, input: &str) {
      if let Result::Err(e) = self.load(input) {
         panic!("{}", e);
      }
   }
   pub fn f_load(&mut self, filename: &str) {
      let mut p = String::new();
      let mut file = File::open(filename).expect(&format!("Policy::f_load: error opening file {}", filename));
      file.read_to_string(&mut p).expect("Policy::f_load: unable to read to string");
      self.s_load(&p);
   }
   pub fn pre(&mut self, input: &str) -> Result<StringSlice, String> {
      let context = Context::new(Rc::new(self.symbols.clone()));
      let input = StringSlice::new(input.to_string());
      if self.symbols.contains_key("::pre") {
         Result::Ok(StringSlice::new(
            eval_parse(context.clone(), "::pre", input)?.to_string()
         ))
      } else {
         Result::Ok(input)
      }    
   }
   pub fn infer(&mut self, input: StringSlice) -> Result<Vec<Rhs>,String> {
      let rhs = parse_rhs(input)?;
      Result::Ok(rhs)
   }
   pub fn hard(&mut self, input: &str) -> Result<Rhs,String> {
      let context = Context::new(Rc::new(self.symbols.clone()));
      let input = self.pre(input)?;
      let program = self.infer(input)?;
      Result::Ok( eval_rhs(context.clone(), &program)? )
   }
   pub fn soft(&mut self, input: &str) -> Result<Rhs,String> {
      let input = self.pre(input)?;
      let program = self.infer(input)?;
      Result::Ok( Rhs::naked(program) )
   }
   pub fn s_hard(&mut self, input: &str) -> String {
      match self.hard(input) {
         Result::Ok(r) => r.to_string(),
         Result::Err(e) => e
      }
   }
   pub fn s_soft(&mut self, input: &str) -> String {
      match self.soft(input) {
         Result::Ok(r) => r.to_string(),
         Result::Err(e) => e
      }
   }
}

