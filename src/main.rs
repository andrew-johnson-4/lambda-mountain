
fn main() {
   for arg in std::env::args().skip(1) {
      if arg.ends_with(".lm") {
         println!("compile file: {}", arg);
      }
   }
}
