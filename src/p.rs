/*

Copyright 2023 - Andrew Johnson

This code and all related intellectual property is available under the terms of
the attached permissive MIT license. This license is intended only to protect
the future development of the project while otherwise allowing people to use
the code and IP as they would like. Please, just be nice.

P: A fast String to AST parser

*/

use crate::*;

fn parse_one_expression(input: &str) -> S {
   let input = input.trim();
   if input.starts_with("λ") {
      if let Some((lhs,rhs)) = input.strip_prefix("λ").unwrap().split_once(".") {
         lambda( parse_expression(lhs), parse_expression(rhs) )
      } else { s_nil() }
   } else if input.len()>=2 && input.starts_with("/") && input.ends_with("/") {
      let input = input.strip_prefix("/").unwrap().strip_suffix("/").unwrap();      
      regex(input)
   } else if input.starts_with("(") && input.ends_with(")") {
      let input = input.strip_prefix("(").unwrap().strip_suffix(")").unwrap();
      parse_expression(input)
   } else {
      let cs = input.to_string();
      let c = cs.chars().next().unwrap();
      if c.is_alphabetic() && !c.is_uppercase() {
         variable(&cs)
      } else {
         literal(&cs)
      }
   }
}

//All strings are valid expressions, this function is total
pub fn parse_expression(input: &str) -> S {
   let input = input.trim();
   let mut depth = 0;
   let mut buf = String::new();
   let mut terms = Vec::new();
   for c in input.chars() {
      if c=='(' { depth += 1; buf.push(c); }
      else if c==')' { depth -= 1; buf.push(c); }
      else if depth==0 && c=='λ' { depth += 1; buf.push(c); }
      else if c==' ' {
         if depth>0 { buf.push(c); }
         else if buf.len()>0 {
            terms.push(parse_one_expression(&buf));
            buf = String::new();
         }
      } else { buf.push(c); }
   }
   if buf.len()>0 { terms.push(parse_one_expression(&buf)) }
   if terms.len()==0 { return s_nil(); }
   let mut f = terms[0].clone();
   for x in &terms[1..] {
      f = app( f, x.clone() );
   }
   f
}

//All strings are valid programs, this function is total
pub fn parse_program(s: &str) -> S {
   let mut kvs = Vec::new();
   for line in s.split("\n") {
   if let Some((l,r)) = line.split_once(":=") {
      kvs.push(( s_atom(l), parse_expression(r) ));
   }}
   kv(&kvs)
}
