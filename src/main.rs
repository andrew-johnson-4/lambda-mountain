use std::fs::File;
use std::io::prelude::*;

use lambda_mountain::Policy;

fn main() {
   let mut policy = Policy {};
   let mut hard = true;
   for arg in std::env::args().skip(1) {
      if arg=="hard" { hard = true; }
      else if arg=="soft" { hard = false; }
      else         { load_policy(&mut policy, &arg); }
   }
   if hard { eval_hard(&policy); }
   else { eval_soft(&policy); }
}

fn load_policy(policy: &mut Policy, filename: &str) {
   let mut p = String::new();
   let mut file = File::open(filename).expect("load_policy: error opening file");
   file.read_to_string(&mut p).expect("load_policy: unable to read to string");
   println!("load_policy: {p}");
}

fn eval_soft(policy: &Policy) {
   println!("eval soft\n");
}

fn eval_hard(policy: &Policy) {
   println!("eval hard\n");
}
