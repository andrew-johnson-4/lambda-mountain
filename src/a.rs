/*

Copyright 2023 - Andrew Johnson

This code and all related intellectual property is available under the terms of
the attached permissive MIT license. This license is intended only to protect
the future development of the project while otherwise allowing people to use
the code and IP as they would like. Please, just be nice.

A: An S-Expression based AST

*/

use regex::Regex;
use std::collections::HashMap;
use itertools::Itertools;
use crate::*;

pub fn literal(s: &str) -> S {
   s_cons( s_atom("literal"), s_atom(s) )
}

pub fn variable(s: &str) -> S {
   s_cons( s_atom("variable"), s_atom(s) )
}

pub fn typ(s: &str) -> S {
   s_cons( s_atom("type"), s_atom(s) )
}

pub fn lambda(l: S, r: S) -> S {
   s_cons( s_atom("lambda"), s_cons(l,r) )
}

pub fn app(f: S, x: S) -> S {
   s_cons( s_atom("app"), s_cons(f,x) )
}

pub fn regex(r: &str) -> S {
   s_cons( s_atom("regex"), s_atom(r) )
}

pub fn list(s: &[S]) -> S {
   let mut tail = s_nil();
   for x in s.iter().rev() {
      tail = s_cons( x.clone(), tail );
   }
   tail
}

pub fn kv(s: &[(S,S)]) -> S {
   let mut tail = s_nil();
   for (k,v) in s.iter().rev() {
      tail = s_cons( s_cons(k.clone(),v.clone()), tail );
   }
   s_cons( s_atom("kv"), tail )
}

pub fn kv_iter(s: &S) -> Vec<(S,S)> {
   let mut kvs = Vec::new();
   let mut h = s.clone();
   while is_cons(&h) {
      let kv = head(&h);
      if is_cons(&kv) {
         kvs.push(( head(&kv), tail(&kv) ));
      }
      h = tail(&h);
   }
   kvs
}

pub fn kv_merge(l: &S, r: &S) -> S {
   let mut kvs = Vec::new();
   for (k,v) in kv_iter(l) {
      kvs.push(( k, v ));
   }
   for (k,v) in kv_iter(r) {
      kvs.push(( k, v ));
   }
   kv(&kvs)
}

pub fn kv_ctx(s: &S) -> HashMap<String,S> {
   let mut ctx = HashMap::new();
   for (k,v) in kv_iter(s) {
   if head(&k).to_string()=="variable" {
      let k = tail(&k).to_string();
      ctx.insert( k, v );
   }}
   ctx
}

pub fn kv_lookup(ctx: &S, key: &S, default: &S) -> S {
   for (k,v) in kv_iter(ctx) {
      if k==*key { return v.clone(); }
   }
   default.clone()
}

pub fn kv_s(ctx: &HashMap<String,S>) -> S {
   kv(&ctx.iter().sorted_by_key(|x| x.0).map(|(k,v)|(variable(k),v.clone())).collect::<Vec<(S,S)>>())
}

pub fn destructure(ctx: &mut HashMap<String,S>, pattern: S, value: S) -> bool {
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
   if is_atom(&head(&pattern)) && head(&pattern).to_string()=="regex" {
      if !is_atom(&head(&value)) || head(&value).to_string()!="literal" { return false; }
      let value = tail(&value).to_string();
      let re = Regex::new(&tail(&pattern).to_string()).unwrap();
      return if let Some(c) = re.captures(&value) {
         for (ci,cm) in c.iter().enumerate() {
            if let Some(m) = cm.map(|m| m.as_str()) {
               let k = "{".to_string() + &format!("{}",ci) + "}";
               ctx.insert( k, literal(m) );
            }
         }
         true
      } else { false }
   }
   if is_atom(&head(&pattern)) && head(&pattern).to_string()=="kv" &&
      is_atom(&head(&value)) && head(&value).to_string()=="kv" {
      for (lk,lv) in kv_iter(&tail(&pattern)) {
         let mut found = false;
         for (rk,rv) in kv_iter(&tail(&value)) {
            if lk==rk {
               if !destructure(ctx, lv, rv) { return false; }
               found = true;
               break;
            }
         }
         if !found { return false; }
      }
      return true;
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
