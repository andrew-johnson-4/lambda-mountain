use std::io;
use std::io::prelude::*;

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
      let context = Context::new_with_policy(policy);

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
            let r = eval_rhs(context.clone(), &program);
            println!("{}", r.to_string());
         }}
      }
   }
}
