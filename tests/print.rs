use std::fs::File;
use std::io::prelude::*;

use lambda_mountain::Policy;

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
   let soft = p.soft(&is);

   assert_eq!( p.soft(&is), "::program print \"hello_world\"" )
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
   let hard = p.hard(&is);

   assert_eq!( hard, "TODO" )
}
