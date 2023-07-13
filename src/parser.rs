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
   let input = input.string[input.start..input.end].trim();
   if let Some((symbol,rhs)) = input.split_once(":=") {
      let rhs = parse_rhs(StringSlice::new(rhs.to_string()));
      if rhs.len()==1 {
         ( symbol.trim().to_string(), rhs[0].clone() )
      } else {
         panic!("Syntax Error: {}", input)
      }
   } else {
      panic!("Syntax Error: {}", input)
   }
}

pub fn parse_rhs(input: StringSlice) -> Vec<Rhs> {
   let input = input.string[input.start..input.end].trim();

   if input.starts_with("λ") {
      if let Some((lhs,rhs)) = input["λ".len()..].split_once(".") {
         vec![Rhs::Lambda(
            parse_lhs(StringSlice::new(lhs.to_string())),
            parse_rhs(StringSlice::new(rhs.to_string()))
         )]
      } else {
         panic!("Syntax Error: {}", input)
      }
   } else {
      unimplemented!("parser::parse_rhs {}", input)
   }

   /*
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
   */

}

//parse_lhs is same as parse_rhs minus the lambda rule
pub fn parse_lhs(input: StringSlice) -> Vec<Lhs> {
   unimplemented!("parser::parse_lhs")
}
