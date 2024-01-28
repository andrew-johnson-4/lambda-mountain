/*

Copyright 2023 - Andrew Johnson

This code and all related intellectual property is available under the terms of
the attached permissive MIT license. This license is intended only to protect
the future development of the project while otherwise allowing people to use
the code and IP as they would like. Please, just be nice.

E: An AST Expression Evaluator

*/

use crate::*;

pub fn eval_soft(s: &S) -> S {
   ctx_eval_soft(&s_nil(), s)
}

pub fn ctx_eval_soft(ctx: &S, s: &S) -> S {
   if !is_cons(s) {
      s.clone()
   } else if head(&s).to_string()=="Variable" {
      let k = tail(&s);
      let v = kv_lookup( ctx, &k, &s );
      if s != &v {
          ctx_eval_soft(ctx, &v)
      } else {
          v
      }
   } else if head(&s).to_string()=="App" {
      let fx = tail(&s);
      let f = ctx_eval_soft(ctx, &head(&fx));
      let x = ctx_eval_soft(ctx, &tail(&fx));
      if head(&f).to_string()=="Lambda" {
         let fl = head(&tail(&f));
         let fr = tail(&tail(&f));
         let mut inner_ctx = kv_ctx(ctx);
         destructure(&mut inner_ctx, fl, x);
         return ctx_eval_soft(&kv_s(&inner_ctx), &fr);
      }
      app(f, x)
   } else { s.clone() }
}
