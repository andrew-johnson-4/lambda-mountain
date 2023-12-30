
use lambda_mountain::*;
use std::fs::File;
use std::io::Read;

fn main() {
   for arg in std::env::args().skip(1) {
      if arg.ends_with(".lm") {
         let mut file = File::open(&arg).unwrap();
         let mut file_contents = String::new();
         file.read_to_string(&mut file_contents).unwrap();
         let s = parse_program(&file_contents);
         compile("", &s);
      }
   }
}
