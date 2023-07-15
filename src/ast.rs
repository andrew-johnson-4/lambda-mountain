use std::rc::Rc;

#[derive(Clone)]
pub struct StringSlice {
   pub string: Rc<String>,
   pub start: usize,
   pub end: usize,
}
impl std::fmt::Display for StringSlice {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "{}", &self.string[self.start..self.end])
   }
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
   pub fn chars(&self) -> std::str::Chars {
      self.string[self.start..self.end].chars()
   }
   pub fn split<'a>(&'a self, sep: &'a str) -> Vec<String> {
      if sep != "\n" {
         return self.string[self.start..self.end].split(sep).map(|s|s.to_string()).collect::<Vec<String>>();
      }
      let mut r = Vec::new();
      let mut s = String::new();
      let mut nest_level = 0;
      for c in self.string[self.start..self.end].chars() {
         if c=='\n' && nest_level > 0 {
            //ignore
         } else if c=='(' {
            nest_level += 1;
            s.push('(');
         } else if c==')' {
            nest_level -= 1;
            s.push(')');
         } else if nest_level <= 0 && c=='\n' {
            nest_level = 0;
            r.push(s);
            s = String::new();
         } else {
            s.push(c);
         }
      }
      r.push(s);
      r
   }
   pub fn split_once<'a>(&'a self, sep: &'a str) -> Option<(&'a str, &'a str)> {
      self.string[self.start..self.end].split_once(sep)
   }
   pub fn trim(&self) -> StringSlice {
      let mut s = self.clone();
      while s.start < s.end && (
         s.string.as_bytes()[s.start] == b' ' ||
         s.string.as_bytes()[s.start] == b'\t' ||
         s.string.as_bytes()[s.start] == b'\n' ||
         s.string.as_bytes()[s.start] == b'\r'
      ) {
         s.start += 1;
      }
      while s.start < s.end && (
         s.string.as_bytes()[s.end-1] == b' ' ||
         s.string.as_bytes()[s.end-1] == b'\t' ||
         s.string.as_bytes()[s.end-1] == b'\n' ||
         s.string.as_bytes()[s.end-1] == b'\r'
      ) {
         s.end -= 1;
      }
      s
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
      &self.string.as_bytes()[self.start..self.start+v.len()] == v.as_bytes()
   }
   pub fn ends_with(&self, v: &str) -> bool {
      self.len() >= v.len() &&
      &self.string.as_bytes()[(self.end-v.len())..self.end] == v.as_bytes()
   }
   pub fn to_string(&self) -> String {
      self.string[self.start..self.end].to_string()
   }
}

#[derive(Clone)]
pub enum Rhs {
   Literal(String),
   Variable(String),
   App(Vec<Rhs>),
   Lambda(Vec<Rhs>,Vec<Rhs>),
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
impl Rhs {
   pub fn naked(rhs: Vec<Rhs>) -> Rhs {
      if rhs.len()==1 {
         rhs[0].clone()
      } else {
         Rhs::App(rhs)
      }
   }
   pub fn as_vec(&self) -> Vec<Rhs> {
      if let Rhs::App(rhs) = self {
         rhs.clone()
      } else {
         vec![self.clone()]
      }
   }
}
