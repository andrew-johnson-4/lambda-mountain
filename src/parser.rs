use crate::ast::*;

pub fn parse_program(input: StringSlice) -> Result<Vec<(String,Rhs)>,String> {
   let mut program = Vec::new();
   for line in input.split('\n') {
      let line = line.trim();
      if line.starts_with("#") {}
      else if line=="" {}
      else if line.contains(":=") {
         program.push(parse_binding( StringSlice::new(line.to_string()) )?);
      } else {
         let e = parse_many_rhs(StringSlice::new(line.to_string()))?;
         let e = if e.len()==1 {
            e[0].clone()
         } else {
            Rhs::App(e)
         };
         program.push(("".to_string(), e));
      }
   }
   Result::Ok( program )
}

pub fn parse_binding(input: StringSlice) -> Result<(String,Rhs),String> {
   let input = input.trim();
   if let Some((symbol,rhs)) = input.split_once(":=") {
      let rhs = parse_many_rhs(StringSlice::new(rhs.to_string()))?;
      if rhs.len()==1 {
         Result::Ok( ( symbol.trim().to_string(), rhs[0].clone() ) )
      } else {
         Result::Err( format!("Syntax Error: {}", input) )
      }
   } else {
      Result::Err( format!("Syntax Error: {}", input) )
   }
}

pub fn parse_many_rhs(input: StringSlice) -> Result<Vec<Rhs>,String> {
   let input = input.trim();
   if input.starts_with("λ") {
      return Result::Ok(vec![ parse_one_rhs(input)? ]);
   }
   let mut many = Vec::new();
   for term in input.split(' ') {
      let term = StringSlice::new(term);
      many.push( parse_one_rhs(term)? );
   }
   Result::Ok(many)
}

pub fn parse_one_rhs(input: StringSlice) -> Result<Rhs,String> {
   let input = input.trim();
   if input.starts_with("λ") {
      if let Some((lhs,rhs)) = input.after("λ").split_once(".") {
         Result::Ok( Rhs::Lambda(
            parse_many_rhs(StringSlice::new(lhs.to_string()))?,
            parse_many_rhs(StringSlice::new(rhs.to_string()))?
         ) )
      } else {
         Result::Err( format!("Syntax Error: {}", input) )
      }
   } else if input.starts_with("(λ") && input.ends_with(")") {
      let input = input.after("(").before(")");
      parse_one_rhs(input)
   } else if input.starts_with("(") && input.ends_with(")") {
      let input = input.after("(").before(")");
      Result::Ok(Rhs::App( parse_many_rhs(input)? ))
   } else {
      let cs = input.to_string();
      let c = cs.chars().next().unwrap();
      if cs.starts_with("::") {
         Result::Ok(Rhs::Variable(cs))
      } else if c.is_alphabetic() && !c.is_uppercase() {
         Result::Ok(Rhs::Variable(cs))
      } else {
         Result::Ok(Rhs::Literal(cs))
      }
   }
}
