use std::collections::HashMap;

pub struct Policy {
   symbols: HashMap<String,Vec<Term>>,
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
                  let term = parse_term(term);
                  if !self.symbols.contains_key(symbol) {
                     self.symbols.insert(symbol.to_string(), Vec::new());
                  }
                  self.symbols.get_mut(symbol).expect("Policy::load")
                      .push(term);
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
      for (k,vs) in self.symbols.iter() {
         for v in vs.iter() {
            println!("{} := {}", k, v.to_string())
         }
      }
      println!("#input:\n{input}\n");
   }
}

pub enum LHS {
   Symbol(String),
   Plural(Vec<LHS>),
   App(Vec<LHS>),
}
impl LHS {
   pub fn to_string(&self) -> String {
      match self {
         LHS::Symbol(s) => s.clone(),
         LHS::Plural(ps) => ps.iter().map(|l| l.to_string()).collect::<Vec<String>>().join(" "),
         LHS::App(ps) => format!("({})", ps.iter().map(|l| l.to_string()).collect::<Vec<String>>().join(" ") ),
      }
   }
}

pub enum Term {
   Variable(String),
   Lambda(LHS,Box<Term>),
   App(Vec<Term>),
}
impl Term {
   pub fn to_string(&self) -> String {
      match self {
         Term::Variable(s) => s.clone(),
         Term::Lambda(lhs,rhs) => format!("λ{}.{}", lhs.to_string(), rhs.to_string()),
         Term::App(ps) => format!("({})", ps.iter().map(|l| l.to_string()).collect::<Vec<String>>().join(" ") ),
      }
   }
}

pub fn parse_lhs(s: &str) -> LHS {
   let s = s.trim();
   if !s.contains(" ") {
      return LHS::Symbol(s.to_string());
   }
   if s.starts_with("(") && s.ends_with(")") {
      return match parse_lhs(&s[1..s.len()-1]) {
         LHS::Plural(lhs) => { LHS::App(lhs) },
         LHS::App(lhs) => { LHS::App(lhs) },
         symbol => { LHS::App(vec![symbol]) },
      };
   }
   let mut nesting_level = 0;
   let mut tokens = Vec::new();
   let mut token = String::new();
   for c in s.chars() {
      if c==' ' && nesting_level==0 {
         tokens.push(parse_lhs(&token));
         token = String::new();
      } else if c=='(' {
         nesting_level += 1;
         token.push(c);
      } else if c==')' {
         if nesting_level==0 {
            panic!("Syntax Error: {lhs}", lhs=s)
         }
         nesting_level -= 1;
         token.push(c);
      } else {
         token.push(c);
      }
   }
   tokens.push(parse_lhs(&token));
   LHS::Plural(tokens)
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
   } else if s.starts_with("(") && s.ends_with(")") {
      parse_term(&s[1..s.len()-1])
   } else if !s.contains(" ") {
      Term::Variable(s.to_string())
   } else {
      let mut nesting_level = 0;
      let mut tokens = Vec::new();
      let mut token = String::new();
      for c in s.chars() {
         if c==' ' && nesting_level==0 {
            tokens.push(parse_term(&token));
            token = String::new();
         } else if c=='(' {
            nesting_level += 1;
            token.push(c);
         } else if c==')' {
            if nesting_level==0 {
               panic!("Syntax Error: {term}", term=s)
            }
            nesting_level -= 1;
            token.push(c);
         } else {
            token.push(c);
         }
      }
      tokens.push(parse_term(&token));
      Term::App(tokens)
   }
}
