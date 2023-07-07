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
      let input = if self.symbols.contains_key("::pre") {
         self.parse("::pre", input).to_string()
      } else { input.to_string() };
      println!("Policy::soft parsed input:\n{input}\n");
   }
   pub fn parse(&mut self, rule: &str, input: &str) -> &str {
      if !self.symbols.contains_key(rule) {
         panic!("unable to apply non-existent rule: {}", rule)
      }
      for term in self.symbols.get(rule).unwrap() {
      if let Term::Lambda(pat,body) = term {
         unimplemented!("Policy::parse λ{}. {}", pat, body)
      } else {
         panic!("unable to apply non-lambda rule: {}", rule)
      }}
      "" //default case is empty string
   }
}

pub enum LHS {
   Symbol(String),
   Plural(Vec<LHS>),
   App(Vec<LHS>),
}
impl std::fmt::Display for LHS {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
         LHS::Symbol(s) => write!(f, "{}", s),
         LHS::Plural(ps) => write!(f, "{}", ps.iter().map(|l| l.to_string()).collect::<Vec<String>>().join(" ") ),
         LHS::App(ps) => write!(f, "({})", ps.iter().map(|l| l.to_string()).collect::<Vec<String>>().join(" ") ),
      }
   }
}

pub enum Term {
   Variable(String),
   Lambda(LHS,Box<Term>),
   App(Vec<Term>),
}
impl std::fmt::Display for Term {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
         Term::Variable(s) => write!(f, "{}", s),
         Term::Lambda(lhs,rhs) => write!(f, "λ{}.{}", lhs, rhs),
         Term::App(ps) => write!(f, "({})", ps.iter().map(|l| l.to_string()).collect::<Vec<String>>().join(" ") ),
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
