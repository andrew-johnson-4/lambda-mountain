
use lambda_mountain::*;
use std::fs::File;
use std::io::Read;

fn main() {
   let mut target = "a.out".to_string();
   let mut option = "".to_string();
   let mut debug = false;
   for arg in std::env::args().skip(1) {
      if option == "-o" {
         target = arg;
         option = "".to_string();
      } else if option == "-p" {
         let mut file = File::open(&arg).unwrap();
         let mut file_contents = String::new();
         file.read_to_string(&mut file_contents).unwrap();
         let s = parse_program(&file_contents);
         println!("{}", s);
      } else if arg=="--debug" {
         debug = true;
      } else if arg.starts_with("-") {
         option = arg;
      } else if arg.ends_with(".lm") {
         let mut file = File::open(&arg).unwrap();
         let mut file_contents = String::new();
         file.read_to_string(&mut file_contents).unwrap();
         let s = parse_program(&file_contents);
         compile(debug, &target, &s);
      }
   }
}
