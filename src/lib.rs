
pub struct Policy {
}

impl Policy {
   pub fn new() -> Policy {
      Policy {}
   }
   pub fn load(&mut self, p: &str) {
      println!("Policy::load\n{p}\n");
   }
   pub fn hard(&mut self, p: &str) {
      println!("Policy::hard\n");
   }
   pub fn soft(&mut self, p: &str) {
      println!("Policy::soft\n");
   }
}
