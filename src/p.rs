/*

Copyright 2023 - Andrew Johnson

This code and all related intellectual property is available under the terms of
the attached permissive MIT license. This license is intended only to protect
the future development of the project while otherwise allowing people to use
the code and IP as they would like. Please, just be nice.

P: A fast String to AST parser

*/

use crate::*;

pub fn parse_expression(s: &str) -> S {
   s_nil()
}

pub fn parse_program(s: &str) -> S {
   s_nil()
}

pub fn parse_many_rhs(input: &str) -> S {
   s_nil()
}

pub fn parse_one_rhs(input: &str) -> S {
   let input = input.trim();
   if input.starts_with("λ") {
      if let Some((lhs,rhs)) = input.strip_prefix("λ").unwrap().split_once(".") {
         lambda( parse_many_rhs(lhs), parse_many_rhs(rhs) )
      } else { s_nil() }
   } else if input.starts_with("(") && input.ends_with(")") {
      let input = input.strip_prefix("(").unwrap().strip_suffix(")").unwrap();
      parse_many_rhs(input)
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
