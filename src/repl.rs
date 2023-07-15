use std::io;
use std::io::prelude::*;
use std::rc::Rc;

use crate::ast::*;
use crate::policy::*;
use crate::parser::*;
use crate::evaluator::*;

pub fn repl_start_session(policy: &mut Policy) {
   loop {
      print!("> ");
      let _ = io::stdout().flush();
      let mut input = String::new();
      io::stdin().read_line(&mut input).expect("Error reading from stdin");
      let context = Context::new(Rc::new(policy.symbols.clone()));

      if input.contains(":=") {
         let input = StringSlice::new(input);
         match parse_binding(input) { Result::Err(e) => {
            println!("{}", e);
         }, Result::Ok((k,v)) => {
            policy.bind(&k,v);
         }}
      } else {
         let input = StringSlice::new(input);
         match parse_many_rhs(input) { Result::Err(e) => {
            println!("{}", e);
         }, Result::Ok(program) => {
            match eval_rhs(context.clone(), &program) { Result::Err(e) => {
               println!("{}", e);
            }, Result::Ok(r) => {
               println!("{}", r.to_string());
            }}
         }}
      }
   }
}
