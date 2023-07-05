use std::fs::File;
use std::io::prelude::*;

use lambda_mountain::Policy;

fn load_policy(policy: &mut Policy, filename: &str) {
   let mut p = String::new();
   policy.load(&p);
}

#[test]
fn print_soft() {
   let mut p = Policy::new();

   let mut ps = String::new();
   let mut file = File::open("examples/print.lm").expect("load_policy: error opening file");
   file.read_to_string(&mut ps).expect("load_policy: unable to read to string");
   p.load(&ps);

   let mut is = String::new();
   let mut file = File::open("examples/print.txt").expect("load_policy: error opening file");
   file.read_to_string(&mut is).expect("load_policy: unable to read to string");
   p.soft(&is);
}

#[test]
fn print_hard() {
   let mut p = Policy::new();

   let mut ps = String::new();
   let mut file = File::open("examples/print.lm").expect("load_policy: error opening file");
   file.read_to_string(&mut ps).expect("load_policy: unable to read to string");
   p.load(&ps);

   let mut is = String::new();
   let mut file = File::open("examples/print.txt").expect("load_policy: error opening file");
   file.read_to_string(&mut is).expect("load_policy: unable to read to string");
   p.hard(&is);
}
