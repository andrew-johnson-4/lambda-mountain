
pub struct Policy {
}

impl Policy {
   pub fn new() -> Policy {
      Policy {}
   }
   pub fn load(&mut self, p: &str) {
      let mut line = String::new();
      for c in p.chars() {
         if c=='\n' {
            if line.len()>0 && !line.starts_with("#") {
               println!("parse line: {line}");
               unimplemented!("Policy::load line");
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
