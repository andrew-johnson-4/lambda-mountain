use crate::ast::*;

pub fn parse_program(input: StringSlice) -> Result<Vec<(String,Rhs)>,String> {
   let mut program = Vec::new();
   for line in input.split("\n") {
      let line = line.trim();
      if line.starts_with("#") {}
      else if line=="" {}
      else {
         program.push(parse_binding( StringSlice::new(line.to_string()) )?);
      }
   }
   Result::Ok( program )
}

pub fn parse_binding(input: StringSlice) -> Result<(String,Rhs),String> {
   let input = input.trim();
   if let Some((symbol,rhs)) = input.split_once(":=") {
      let rhs = parse_rhs(StringSlice::new(rhs.to_string()))?;
      if rhs.len()==1 {
         Result::Ok( ( symbol.trim().to_string(), rhs[0].clone() ) )
      } else {
         Result::Err( format!("Syntax Error: {}", input) )
      }
   } else {
      Result::Err( format!("Syntax Error: {}", input) )
   }
}

pub fn parse_rhs(input: StringSlice) -> Result<Vec<Rhs>,String> {
   let input = input.trim();

   if input.len()==0 {
      Result::Ok( Vec::new() )
   } else if input.starts_with("λ") {
      if let Some((lhs,rhs)) = input.after("λ".len()).split_once(".") {
         Result::Ok( vec![Rhs::Lambda(
            parse_rhs(StringSlice::new(lhs.to_string()))?,
            parse_rhs(StringSlice::new(rhs.to_string()))?
         )] )
      } else {
         Result::Err( format!("Syntax Error: {}", input) )
      }
   } else if input.starts_with("(λ") {
      let mut nest_level = 1;
      let mut inner = String::new();
      for c in input.chars().skip(1) {
         if nest_level==0 {
         } else if c=='(' {
            inner.push('(');
            nest_level += 1;
         } else if c==')' {
            nest_level -= 1;
            if nest_level > 0 {
               inner.push(')');
            }
         } else {
            inner.push(c);
         }
      }
      parse_rhs(StringSlice::new(inner))
   } else if input.starts_with("(") {
      let mut app = String::new();
      let mut rem = String::new();
      let mut nest_level = 1;
      for c in input.chars().skip(1) {
         if nest_level==0 {
            rem.push(c);
         } else if c=='(' {
            app.push('(');
            nest_level += 1;
         } else if c==')' {
            nest_level -= 1;
            if nest_level > 0 {
               app.push(')');
            }
         } else {
            app.push(c);
         }
      }
      let mut cs = parse_rhs(StringSlice::new(rem))?;
      cs.insert(0, Rhs::App( parse_rhs(StringSlice::new(app))? ) );
      Result::Ok( cs )
   } else {
      let (id,cs) = if let Some((id,cs)) = input.split_once(" ") {
         (id.to_string(), cs.to_string())
      } else {
         (input.to_string(), "".to_string())
      };
      let cs = StringSlice::new(cs);
      let mut cs = parse_rhs(cs)?;
      let c = id.chars().collect::<Vec<char>>();
      let c = c.first().unwrap();
      if c.is_alphabetic() && !c.is_uppercase() {
         cs.insert(0, Rhs::Variable(id));
      } else {
         cs.insert(0, Rhs::Literal(id));
      }
      Result::Ok( cs )
   }
}
