use crate::{StringSlice, parse_program, Rhs, Context, eval_rhs};
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
impl Rule {
   pub fn to_string(&self) -> String {
      let mut line = "".to_string();
      for (i,s) in self.string.iter().enumerate() {
         if i>0 { line += " "; }
         line += &s.to_string();
      }
      line
   }
}

//Symbols in Production Rules
#[derive(Clone)]
pub enum Symbol {
   Bind(String,Box<Symbol>),
   Regex(String),
   Scan(String,String,String),
   Descend(String),
}
impl Symbol {
   pub fn to_string(&self) -> String {
      match self {
         Symbol::Bind(x,y) => format!("Bind({},{})", x, y.to_string()),
         Symbol::Regex(r) => format!("Regex({})", r),
         Symbol::Scan(x,y,z) => format!("Scan({},{},{})", x, y, z),
         Symbol::Descend(x) => format!("Descend({})", x),
      }
   }
}

pub enum ParseResult {
   Result(Rhs,Input),
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
   ctx: Context,
   data: Rc<String>,
   line_no: usize,
   column_no: usize,
   offset_start: usize,
}
impl Input {
   fn bind(&self, k: String, v: Rhs) -> Input {
      Input {
         ctx: self.ctx.clone().bind(k,v),
         data: self.data.clone(),
         line_no: self.line_no,
         column_no: self.column_no,
         offset_start: self.offset_start,
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
   fn run_local_symbol(&mut self, sym: &Symbol, mut input: Input) -> ParseResult {
      match sym {
         Symbol::Bind(l,r) => {
            match self.run_local_symbol(&r,input.clone()) {
               ParseResult::Result(v,i) => {
                  let i = i.bind(l.clone(), v.clone());
                  ParseResult::Result(v,i)   
               },
               err => err,
            }
         },
         Symbol::Regex(p) => {
            if !self.regexes.contains_key(p) {
               let re = Regex::new(&p).expect(&format!("Could not compile regular expression: {}", p));
               self.regexes.insert(p.clone(), re);
            }
            let re = self.regexes.get(p).unwrap();
            if let Some(m) = re.find_at(&input.data[input.offset_start..], 0) {
               let m = m.as_str().to_string();
               input.offset_start += m.len();
               input.line_no += m.matches("\n").count();
               if m.contains("\n") {
                  input.column_no = m.len() - m.rfind("\n").unwrap();
               } else {
                  input.column_no += m.len();
               }
               ParseResult::Result(Rhs::Literal(m),input)
            } else {
               ParseResult::Error(format!("Expected /{}/ at line {}, column {}", p, input.line_no, input.column_no))
            }
         },
         Symbol::Scan(l,m,r) => unimplemented!("Grammar::run Symbol::Scan({},{},{})", l, m, r),
         Symbol::Descend(d) => {
            self.run_local(d, input)
         },
      }
   }
   fn run_local_rule(&mut self, rule: &Rule, mut input: Input) -> ParseResult {
      for symbol in rule.string.iter() {
         match self.run_local_symbol(symbol, input.clone()) {
            ParseResult::Result(r,i) => { input = i; },
            err => { return err; },
         }
      }
      return ParseResult::Result(
        eval_rhs(input.ctx.clone(), &[rule.retval.clone()]),
        input
      )
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
         ctx: Context::new(),
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
               ParseResult::Result(
                  eval_rhs(input.ctx.clone(), &[retval.clone()]),
                  input
               )
            }
         },
         e => e
      }
   }
}

pub fn compile_rule(grammar: &mut Grammar, rule_name: String, rule: Rhs) -> Symbol {
   match rule {
      Rhs::App(ref rs) => {
         if rs.len() == 3 {
         if let Rhs::Literal(ref rl) = &rs[0] {
         if let Rhs::Variable(ref rv) = &rs[1] {
         if rl == ":" {
            let rt = compile_rule(grammar, rule_name, rs[2].clone());
            return Symbol::Bind(rv.clone(),Box::new(rt));
         }}}}
         unimplemented!("compile_rule unknown rule application: {}", rule)
      },
      Rhs::Literal(s) => {
         let s = s.strip_prefix("/").expect("regex must start with /")
                  .strip_suffix("/").expect("regex must end with /");
         Symbol::Regex(format!("^{}", s))
      },
      Rhs::Variable(s) => {
         Symbol::Descend(s.clone())
      },
      Rhs::Lambda(lhs,rhs) => {
         let mut string = Vec::new();
         for (li,l) in lhs.iter().enumerate() {
            string.push(compile_rule( grammar, format!("{}.{}",rule_name,li), l.clone()));
         }
         if !grammar.rules.contains_key(&rule_name) {
            grammar.rules.insert(rule_name.clone(), Vec::new());
         }
         println!("compile rhs {}", rhs);
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
