
use crate::{StringSlice, parse_program, Rhs};
use std::collections::HashMap;

//Compiled Grammars are just special-case left-hand-sides for term bindings

//Type-0 Grammar Rules
#[derive(Clone)]
struct Rule {
   string: Vec<Symbol>,
   retval: Rhs,
}

//Symbols in Production Rules
#[derive(Clone)]
pub enum Symbol {
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
   rules: HashMap<String,Vec<Rule>>,
}
impl Grammar {
   pub fn new() -> Grammar {
      Grammar {
         rules: HashMap::new(),
      }
   }
   pub fn run(&self, input: &str) -> ParseResult {
      unimplemented!("Grammar::run")
   }
}

pub fn compile_rule(grammar: &mut Grammar, rule_name: String, rule: Rhs) -> Symbol {
   match rule {
      //App(Vec<Rhs>),
      //Poly(Vec<Rhs>),
      Rhs::Literal(s) => Symbol::Regex(s),
      Rhs::Variable(s) => unimplemented!("compile_rule Variable {}", s),
      Rhs::Lambda(lhs,rhs) => {
         let mut string = Vec::new();
         for (li,l) in lhs.iter().enumerate() {
            string.push(compile_rule( grammar, format!("{}.{}",rule_name,li), l.clone()));
         }
         if !grammar.rules.contains_key(&rule_name) {
            grammar.rules.insert(rule_name.clone(), Vec::new());
         }
         grammar.rules.get_mut(&rule_name).expect("parser_generator::compile_rule grammar.rules.get_mut")
                      .push(Rule {
            string: string,
            retval: *rhs,
         });
         Symbol::Descend(rule_name)
      },
      r => unimplemented!("compile_rule unknown {}", r)
   }
}

pub fn compile(input: &str) -> Grammar {
   let input = StringSlice::new(input.to_string());
   let grammar = parse_program(input);
   match grammar {
      Ok(lines) => {
         let mut grammar = Grammar::new();
         for (k,v) in lines {
            compile_rule(&mut grammar, k.clone(), v.clone());
            println!("{} := {}", k, v);
         }
         unimplemented!("parser_generator::compile")
      },
      Err(err) => panic!("{}", err),
   }
}
