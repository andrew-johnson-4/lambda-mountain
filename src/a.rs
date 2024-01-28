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
   s_cons( s_atom("Literal"), s_atom(s) )
}

pub fn variable(s: &str) -> S {
   s_cons( s_atom("Variable"), s_atom(s) )
}

pub fn typ(s: &str) -> S {
   s_cons( s_atom("Type"), s_atom(s) )
}

pub fn local(s: &str) -> S {
   s_cons( s_atom("Local"), s_atom(s) )
}

pub fn ctx_local(k: S, v: S) -> S {
   s_cons( s_atom("Local"),  s_cons(k, v) )
}

pub fn ctx_global(k: S, v: S) -> S {
   s_cons( s_atom("Global"),  s_cons(k, v) )
}

pub fn lambda(l: S, r: S) -> S {
   s_cons( s_atom("Lambda"), s_cons(l,r) )
}

pub fn app(f: S, x: S) -> S {
   s_cons( s_atom("App"), s_cons(f,x) )
}

pub fn nil() -> S {
   s_nil()
}

pub fn list(s: &[S]) -> S {
   let mut tail = s_nil();
   for x in s.iter().rev() {
      tail = s_cons( x.clone(), tail );
   }
   tail
}

pub fn kv_iter(s: &S) -> Vec<(S,S)> {
   let mut kvs = Vec::new();
   let mut h = s.clone();
   while is_cons(&h) {
      let kv = tail(&h);
      let tag = head(&kv).to_string();
      if tag=="Local" || tag=="Global" {
         let kv = tail(&kv);
         let k = head(&kv);
         let v = tail(&kv);
         kvs.push( (k,v) );    
      }
      h = head(&h);
   }
   kvs
}

pub fn kv_merge(l: &S, r: &S) -> S {
   let mut l = l.clone();
   let mut r = r.clone();
   let mut kvs = s_nil();
   while !is_nil(&l) {
      let kv = tail(&l);
      l = head(&l);
      kvs = kv_add( &kvs, &kv );
   }
   while !is_nil(&r) {
      let kv = tail(&r);
      r = head(&r);
      kvs = kv_add( &kvs, &kv );
   }
   kvs
}

pub fn kv_add(root: &S, kv: &S) -> S {
   s_cons( root.clone(), kv.clone() )
}

pub fn kv_lookup(ctx: &S, key: &S, default: &S) -> S {
   for (k,v) in kv_iter(ctx) {
      if k==*key { return v.clone(); }
   }
   default.clone()
}

pub fn kv_ctx(s: &S) -> HashMap<String,S> {
   let mut ctx = HashMap::new();
   for (k,v) in kv_iter(s) {
      let k = k.to_string();
      ctx.insert( k, v );
   }
   ctx
}

pub fn kv_s(ctx: &HashMap<String,S>) -> S {
   kv(&ctx.iter().map(|(k,v)| (s_atom(&k),v.clone())).collect::<Vec<(S,S)>>())
}

pub fn kv(s: &[(S,S)]) -> S {
   let mut root = s_nil();
   for (k,v) in s.iter().rev() {
      root = s_cons( root, ctx_local(k.clone(),v.clone()) );
   }
   root
}

pub fn destructure(ctx: &mut HashMap<String,S>, pattern: S, value: S) -> bool {
   if pattern==value { return true; }
   if !is_cons(&pattern) { return false; }
   if head(&pattern)==s_atom("Variable") {
      let k = tail(&pattern).to_string();
      ctx.insert( k, value );
      return true;
   }
   if !is_cons(&value) { return false; }
   if is_atom(&head(&pattern)) && head(&pattern).to_string()=="Lambda" {
      return false;
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
   if head(&value)==s_atom("Variable") {
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
