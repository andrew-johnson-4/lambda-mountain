use crate::ast::*;

pub fn parse_program(input: StringSlice) -> Vec<(String,Rhs)> {
   let mut program = Vec::new();
   for line in input.split("\n") {
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
   let input = input.trim();
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
   let input = input.trim();

   if input.len()==0 {
      Vec::new()
   } else if input.starts_with("λ") {
      if let Some((lhs,rhs)) = input.after("λ".len()).split_once(".") {
         vec![Rhs::Lambda(
            parse_lhs(StringSlice::new(lhs.to_string())),
            parse_rhs(StringSlice::new(rhs.to_string()))
         )]
      } else {
         panic!("Syntax Error: {}", input)
      }
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
      let mut cs = parse_rhs(StringSlice::new(rem));
      cs.insert(0, Rhs::App( parse_rhs(StringSlice::new(app)) ) );
      cs
   } else {
      let (id,cs) = if let Some((id,cs)) = input.split_once(" ") {
         (id.to_string(), cs.to_string())
      } else {
         (input.to_string(), "".to_string())
      };
      let cs = StringSlice::new(cs);
      let mut cs = parse_rhs(cs);
      if id.chars().collect::<Vec<char>>().first().unwrap().is_alphabetic() {
         cs.insert(0, Rhs::Variable(id));
      } else {
         cs.insert(0, Rhs::Literal(id));
      }
      cs
   }
}

//parse_lhs is same as parse_rhs minus the lambda rule
pub fn parse_lhs(input: StringSlice) -> Vec<Lhs> {
   let input = input.trim();

   if input.len()==0 {
      Vec::new()
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
      let mut cs = parse_lhs(StringSlice::new(rem));
      cs.insert(0, Lhs::App( parse_lhs(StringSlice::new(app)) ) );
      cs
   } else {
      let (id,cs) = if let Some((id,cs)) = input.split_once(" ") {
         (id.to_string(), cs.to_string())
      } else {
         (input.to_string(), "".to_string())
      };
      let cs = StringSlice::new(cs);
      let mut cs = parse_lhs(cs);
      if id.chars().collect::<Vec<char>>().first().unwrap().is_alphabetic() {
         cs.insert(0, Lhs::Variable(id));
      } else {
         cs.insert(0, Lhs::Literal(id));
      }
      cs
   }
}
