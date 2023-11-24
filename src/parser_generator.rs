use crate::{StringSlice, parse_program, Rhs};
use std::collections::HashMap;
use regex::Regex;
use std::rc::Rc;

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
   Result(String,Input),
   Error(String),
}
impl ParseResult {
   pub fn to_string(&self) -> String {
      match self {
         ParseResult::Result(s,_) => { format!("Parse Result: {}", s) },
         ParseResult::Error(s) => { format!("Parse Error: {}", s) },
      }
   }
}

#[derive(Clone)]
pub struct Input {
   data: Rc<String>,
   line_no: usize,
   column_no: usize,
   offset_start: usize,
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
   fn run_local_rule(&mut self, rule: &Rule, mut input: Input) -> ParseResult {
      let mut results = Vec::new();
      for symbol in rule.string.iter() {
      match symbol {
         Symbol::Bind(l,r) => unimplemented!("Grammar::run Symbol::Bind({},{})", l, r),
         Symbol::Regex(p) => {
            if !self.regexes.contains_key(p) {
               let re = Regex::new(p).expect(&format!("Could not compile regular expression: {}", p));
               self.regexes.insert(p.clone(), re);
            }
            let re = self.regexes.get(p).unwrap();
            if let Some(m) = re.find_at(&input.data, input.offset_start) {
               let m = m.as_str().to_string();
               input.offset_start += m.len();
               input.line_no += m.matches("\n").count();
               if m.contains("\n") {
                  input.column_no = m.len() - m.rfind("\n").unwrap();
               } else {
                  input.column_no += m.len();
               }
               results.push(m);
            } else {
               return ParseResult::Error(format!("Expected /{}/ at line {}, column {}", p, input.line_no, input.column_no))
            }
         },
         Symbol::Scan(l,m,r) => unimplemented!("Grammar::run Symbol::Scan({},{},{})", l, m, r),
         Symbol::Descend(r) => unimplemented!("Grammar::run Symbol::Descend({})", r),
      }}
      return ParseResult::Result(results.join(","),input)
   }
   fn run_local(&mut self, rule: &str, input: Input) -> ParseResult {
      let rules = self.rules.get(rule).expect(&format!("Could not find rule {} in grammar",rule)).clone();
      for rule in rules.iter() {
         if let ParseResult::Result(r,i) = self.run_local_rule(rule, input.clone()) {
            return ParseResult::Result(r,i);
         }
      }
      ParseResult::Error(format!("Expected {} at line {}, column {}", rule, input.line_no, input.column_no))
   }
   pub fn run(&mut self, rule: &str, input: &str) -> ParseResult {
      let input = Input {
         data: Rc::new(input.to_string()),
         line_no: 1,
         column_no: 1,
         offset_start: 0,
      };
      match self.run_local(rule, input.clone()) {
         ParseResult::Result(retval,input) => {
            if input.offset_start != input.data.len() {
               ParseResult::Error(format!("Expected EOF at line {}, column {}", input.line_no, input.column_no))      
            } else {
               ParseResult::Result(retval,input)
            }
         },
         e => e
      }
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
