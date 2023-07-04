
pub struct Policy {
}

impl Policy {
   pub fn new() -> Policy {
      Policy {}
   }
   pub fn load(&mut self, p: &str) {
      println!("Policy::load\n{p}\n");
   }
   pub fn hard(&mut self, input: &str) {
      println!("Policy::hard\n{input}\n");
   }
   pub fn soft(&mut self, input: &str) {
      println!("Policy::soft\n{input}\n");
   }
}
