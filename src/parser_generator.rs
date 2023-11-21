
use crate::{StringSlice,parse_program};

//Compiled Grammars are just special-case left-hand-sides for term bindings

//Type-0 Grammar Rules
struct Rule {
   name: String,
   string: Vec<Symbol>,
}

//Symbols in Production Rules
enum Symbol {
   Bind(String,String),
   Regex(String),
   Scan(String,String,String),
   Descend(String),
}

pub struct ParseResult {
}
impl ParseResult {
   pub fn to_string(&self) -> String {
      unimplemented!("ParseResult::to_string")
   }
}

pub struct Grammar {
}
impl Grammar {
   pub fn run(&self, input: &str) -> ParseResult {
      unimplemented!("Grammar::run")
   }
}

pub fn compile(input: &str) -> Grammar {
   let input = StringSlice::new(input.to_string());
   let grammar = parse_program(input);
   match grammar {
      Ok(grammar) => {
         for (k,v) in grammar {
            println!("{} := {}", k, v);
         }
         unimplemented!("parser_generator::compile")
      },
      Err(err) => panic!("{}", err),
   }
}
