use std::fs::File;
use std::io::prelude::*;

fn main() {
   let mut hard = true;
   for arg in std::env::args().skip(1) {
      if arg=="hard" { hard = true; }
      else if arg=="soft" { hard = false; }
      else if hard { eval_hard(&arg); }
      else         { eval_soft(&arg); }
   }
}

fn eval_soft(filename: &str) {
   let mut policy = String::new();
   let mut file = File::open(filename).expect("eval_soft: error opening file");
   file.read_to_string(&mut policy).expect("eval_soft: unable to read to string");
   println!("eval soft:\n{policy}\n");
}

fn eval_hard(filename: &str) {
   let mut policy = String::new();
   let mut file = File::open(filename).expect("eval_hard: error opening file");
   file.read_to_string(&mut policy).expect("eval_hard: unable to read to string");
   println!("eval hard:\n{policy}\n");
}
