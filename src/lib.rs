
pub struct Policy {
}

impl Policy {
   pub fn new() -> Policy {
      Policy {}
   }
   pub fn load(&mut self, p: &str) {
      println!("Policy::load\n{p}\n");
      unimplemented!("Policy::load");
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
