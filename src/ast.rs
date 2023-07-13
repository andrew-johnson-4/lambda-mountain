use std::rc::Rc;

#[derive(Clone)]
pub struct StringSlice {
   pub string: Rc<String>,
   pub start: usize,
   pub end: usize,
}
impl StringSlice {
   pub fn new(s: String) -> StringSlice {
      let s_len = s.len();
      StringSlice {
         string: Rc::new(s),
         start: 0,
         end: s_len,
      }
   }
   pub fn len(&self) -> usize {
      self.end - self.start
   }
   pub fn before(&self, l: usize) -> StringSlice {
      assert!( self.len() >= l );
      StringSlice {
         string: self.string.clone(),
         start: self.start,
         end: self.end - l,
      }
   }
   pub fn after(&self, l: usize) -> StringSlice {
      assert!( self.len() >= l );
      StringSlice {
         string: self.string.clone(),
         start: self.start + l,
         end: self.end,
      }
   }
   pub fn starts_with(&self, v: &str) -> bool {
      self.len() >= v.len() &&
      &self.string[self.start..self.start+v.len()] == v
   }
   pub fn ends_with(&self, v: &str) -> bool {
      self.len() >= v.len() &&
      &self.string[(self.end-v.len())..self.end] == v
   }
   pub fn to_string(&self) -> String {
      self.string[self.start..self.end].to_string()
   }
}

#[derive(Clone)]
pub enum Lhs {
   Literal(String),
   Variable(String),
   App(Vec<Lhs>),
}
impl std::fmt::Display for Lhs {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
         Lhs::Literal(s) => write!(f, "{}", s),
         Lhs::Variable(s) => write!(f, "{}", s),
         Lhs::App(ps) => write!(f, "({})", ps.iter().map(|l| l.to_string()).collect::<Vec<String>>().join(" ") ),
      }
   }
}

#[derive(Clone)]
pub enum Rhs {
   Literal(String),
   Variable(String),
   App(Vec<Rhs>),
   Lambda(Vec<Lhs>,Vec<Rhs>),
   Poly(Vec<Rhs>),
}
impl std::fmt::Display for Rhs {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      match self {
         Rhs::Literal(s) => write!(f, "{}", s),
         Rhs::Variable(s) => write!(f, "{}", s),
         Rhs::App(ps) => write!(f, "({})", ps.iter().map(|l| l.to_string()).collect::<Vec<String>>().join(" ") ),
         Rhs::Lambda(ls,rs) => write!(f, "(λ{}.{})",
            ls.iter().map(|l| l.to_string()).collect::<Vec<String>>().join(" "),
            rs.iter().map(|r| r.to_string()).collect::<Vec<String>>().join(" "),
         ),
         Rhs::Poly(vs) => write!(f, "({})", vs.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" | ") ),
      }
   }
}

