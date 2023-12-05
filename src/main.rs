
fn main() {}

/* TODO bootstrap

use std::fs::File;
use std::io::prelude::*;

use lambda_mountain::{Policy,repl_start_session};

fn main() {
   let mut policy = Policy::new();
   let mut hard = true;
   for arg in std::env::args().skip(1) {
      if arg=="hard" { hard = true; }
      else if arg=="soft" { hard = false; }
      else if arg=="repl" { repl_start_session(&mut policy); }
      else         { load_policy(&mut policy, &arg); }
   }
   if hard { eval_hard(&mut policy); }
   else { eval_soft(&mut policy); }
}

fn load_policy(policy: &mut Policy, filename: &str) {
   let mut p = String::new();
   let mut file = File::open(filename).expect("load_policy: error opening file");
   file.read_to_string(&mut p).expect("load_policy: unable to read to string");
   policy.s_load(&p);
}

fn eval_soft(policy: &mut Policy) {
   let input = "";
   println!("{}", policy.s_soft(input));
}

fn eval_hard(policy: &mut Policy) {
   let input = "";
   println!("{}", policy.s_hard(input));
}

*/
