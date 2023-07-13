use crate::ast::*;

pub fn parse_program(input: StringSlice) -> Vec<(String,Rhs)> {
   let mut program = Vec::new();
   for line in input.string[input.start..input.end].split("\n") {
      let line = line.trim();
      if line.starts_with("#") {}
      else if line=="" {}
      else {
         program.push(parse_binding( StringSlice::new(line.to_string()) ));
      }
   }
   program
}

pub fn parse_binding(input: StringSlice) -> (String,Rhs) {
   unimplemented!("parser::parse_binding {}", &input.string[input.start..input.end])
}

/*
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

pub fn parse_term(s: &str) -> Rhs {
   let s = s.trim();
   if s.starts_with("λ") {
      if let Some((lhs,rhs)) = s[2..].split_once(".") {
         Rhs::Lambda(
            parse_lhs(lhs),
            Box::new( parse_term(rhs) )
         )
      } else {
         panic!("Syntax Error: {term}", term=s)
      }
   } else if s.starts_with("(") && s.ends_with(")") {
      parse_term(&s[1..s.len()-1])
   } else if !s.contains(" ") {
      Rhs::Variable(s.to_string())
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
      Rhs::App(tokens)
   }
}
*/
