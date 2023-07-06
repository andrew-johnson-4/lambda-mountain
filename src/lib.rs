use std::collections::HashMap;

pub struct Policy {
   symbols: HashMap<String,Term>,
}

impl Policy {
   pub fn new() -> Policy {
      Policy {
         symbols: HashMap::new()
      }
   }
   pub fn load(&mut self, p: &str) {
      let mut line = String::new();
      for c in p.chars() {
         if c=='\n' {
            if line.len()>0 && !line.starts_with("#") {
               if let Some((symbol,term)) = line.split_once(" := ") {
                  self.symbols.insert(symbol.to_string(), parse_term(term));
               } else {
                  panic!("Syntax Error: {line}", line=line)
               }
            }
            line = String::new();
         } else {
            line.push(c);
         }
      }
   }
   pub fn hard(&mut self, input: &str) {
      println!("Policy::hard\n{input}\n");
      unimplemented!("Policy::hard");
   }
   pub fn soft(&mut self, input: &str) {
      println!("Policy::soft\n{input}\n");
      unimplemented!("Policy::soft");
   }
}

pub enum LHS {
   Symbol(String),
}

pub enum Term {
   Lambda(LHS,Box<Term>)
}

pub fn parse_lhs(s: &str) -> LHS {
   let s = s.trim();
   println!("parse_lhs: {s}");
   unimplemented!("parse_lhs")
}

pub fn parse_term(s: &str) -> Term {
   let s = s.trim();
   if s.starts_with("λ") {
      if let Some((lhs,rhs)) = s[2..].split_once(".") {
         Term::Lambda(
            parse_lhs(lhs),
            Box::new( parse_term(rhs) )
         )
      } else {
         panic!("Syntax Error: {term}", term=s)
      }
   } else {
      println!("parse_term: {s}");
      unimplemented!("parse_term")
   }
}
