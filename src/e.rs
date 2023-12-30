/*

Copyright 2023 - Andrew Johnson

This code and all related intellectual property is available under the terms of
the attached permissive MIT license. This license is intended only to protect
the future development of the project while otherwise allowing people to use
the code and IP as they would like. Please, just be nice.

E: An AST Expression Evaluator

*/

use crate::*;

pub fn eval(s: &S) -> S {
   ctx_eval(&s_nil(), s)
}

pub fn ctx_eval(ctx: &S, s: &S) -> S {
   if !is_cons(s) { return s.clone(); }
   if head(&s).to_string()=="variable" {
      return kv_lookup( ctx, &s, &s );
   }
   if head(&s).to_string()=="app" {
      let fx = tail(&s);
      let f = head(&fx);
      let x = tail(&fx);
      if head(&f).to_string()=="lambda" {
         let fl = head(&tail(&f));
         let fr = tail(&tail(&f));
         let mut ctx = kv_ctx(ctx);
         destructure(&mut ctx, fl, x);
         return ctx_eval(&kv_s(&ctx), &fr);
      }
   }
   s.clone()
}
