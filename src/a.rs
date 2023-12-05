/*

Copyright 2023 - Andrew Johnson

This code and all related intellectual property is available under the terms of
the attached permissive MIT license. This license is intended only to protect
the future development of the project while otherwise allowing people to use
the code and IP as they would like. Please, just be nice.

A: An S-Expression based AST

*/

use std::collections::HashMap;
use crate::*;

pub fn literal(s: &str) -> S {
   s_cons( s_atom("literal"), s_atom(s) )
}

pub fn variable(s: &str) -> S {
   s_cons( s_atom("variable"), s_atom(s) )
}

pub fn lambda(l: S, r: S) -> S {
   s_cons( s_atom("lambda"), s_cons(l,r) )
}

pub fn regex(r: &str) -> S {
   s_nil()
}

pub fn list(s: &[S]) -> S {
   let mut tail = s_nil();
   for x in s.iter().rev() {
      tail = s_cons( x.clone(), tail );
   }
   tail
}

pub fn kv(s: &[(S,S)]) -> S {
   s_nil()
}

fn destructure(ctx: &mut HashMap<String,S>, pattern: S, value: S) -> bool {
   if pattern==value { return true; }
   if !is_cons(&pattern) { return false; }
   if head(&pattern)==s_atom("variable") {
      let k = tail(&pattern).to_string();
      ctx.insert( k, value );
      return true;
   }
   if !is_cons(&value) { return false; }
   if is_atom(&head(&pattern)) && head(&pattern).to_string()=="lambda" {
      return false;
   }
   destructure(ctx, head(&pattern), head(&value)) &&
   destructure(ctx, tail(&pattern), tail(&value))
}
fn restructure(ctx: &HashMap<String,S>, value: S) -> S {
   if !is_cons(&value) { return value; }
   if head(&value)==s_atom("variable") {
      let k = tail(&value).to_string();
      return if let Some(v) = ctx.get(&k) { v.clone() }
      else { value };
   }
   value
}
pub fn map(lhs: S, v: S, rhs: S) -> S {
   let mut ctx = HashMap::new();
   if destructure(&mut ctx, lhs, v) {
      restructure(&ctx, rhs)
   } else { s_nil() }
}
