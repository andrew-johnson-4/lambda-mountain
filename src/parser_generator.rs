use crate::{StringSlice, parse_program, Rhs};
use std::collections::HashMap;
use regex::Regex;

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

pub enum ParseResult {
   Result(String),
   Error(String),
}
impl ParseResult {
   pub fn to_string(&self) -> String {
      match self {
         ParseResult::Result(s) => { format!("Parse Result: {}", s) },
         ParseResult::Error(s) => { format!("Parse Error: {}", s) },
      }
   }
}

pub struct Grammar {
   regexes: HashMap<String,Regex>,
   rules: HashMap<String,Vec<Rule>>,
}
impl Grammar {
   pub fn new() -> Grammar {
      Grammar {
         rules: HashMap::new(),
         regexes: HashMap::new(),
      }
   }
   fn run_local(&mut self, rule: &str, input: &str, lineno: usize, colno: usize) -> ParseResult {
      for rule in self.rules.get(rule).expect(&format!("Could not find rule {} in grammar",rule)) {
         for symbol in rule.string.iter() {
         match symbol {
            Symbol::Bind(l,r) => unimplemented!("Grammar::run Symbol::Bind({},{})", l, r),
            Symbol::Regex(p) => {
               if !self.regexes.contains_key(p) {
                  println!("Compile regex: {}", p);
                  let re = Regex::new(p).expect(&format!("Could not compile regular expression: {}", p));
                  self.regexes.insert(p.clone(), re);
               }
               let re = self.regexes.get(p).unwrap();
               println!("try regex {} on input '{}'", p, input);
               if let Some(m) = re.find(input) {
                  println!("match regex {} on input '{}'", p, input);
                  return ParseResult::Result(m.as_str().to_string());
               }
            },
            Symbol::Scan(l,m,r) => unimplemented!("Grammar::run Symbol::Scan({},{},{})", l, m, r),
            Symbol::Descend(r) => unimplemented!("Grammar::run Symbol::Descend({})", r),
         }}
      }
      ParseResult::Error(format!("Expected {} at line {}, column {}", rule, lineno, colno))
   }
   pub fn run(&mut self, rule: &str, input: &str) -> ParseResult {
      self.run_local(rule, input, 1, 1)
   }
}

pub fn compile_rule(grammar: &mut Grammar, rule_name: String, rule: Rhs) -> Symbol {
   match rule {
      //App(Vec<Rhs>),
      //Poly(Vec<Rhs>),
      Rhs::Literal(s) => {
         let s = s.strip_prefix("/").expect("regex must start with /")
                  .strip_suffix("/").expect("regex must end with /");
         Symbol::Regex(format!("^{}", s))
      }
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
   //Compilation Target here is just a dynamic Tree definition of the grammar
   //Optimized compilation target should be defined in LM, not Rust
   let input = StringSlice::new(input.to_string());
   let rules = parse_program(input);
   match rules {
      Ok(lines) => {
         let mut grammar = Grammar::new();
         for (k,v) in lines {
            compile_rule(&mut grammar, k.clone(), v.clone());
         }
         grammar
      },
      Err(err) => panic!("{}", err),
   }
}
