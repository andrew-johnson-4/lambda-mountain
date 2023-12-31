/*

Copyright 2023 - Andrew Johnson

This code and all related intellectual property is available under the terms of
the attached permissive MIT license. This license is intended only to protect
the future development of the project while otherwise allowing people to use
the code and IP as they would like. Please, just be nice.

G: A Basic Codegen

*/

use crate::*;
use std::collections::HashMap;

pub fn flatten(output: &mut String, input: &S) {
   if is_cons(input) {
      flatten( output, &head(input) );
      flatten( output, &tail(input) );
   } else if is_atom(input) {
      let l = input.to_string();
      if l == "literal" || l == "variable" || l == "app" {}
      else if l=="\\t" { output.push('\t'); }
      else if l=="\\n" { output.push('\n'); }
      else {
         output.push_str( &l );
         output.push( ' ' );
      }
   }
}

pub fn assemble(cfg: &str, program: &S) {
   let mut code = String::new();
   flatten( &mut code, program );
   unimplemented!("compile_punc:\n{}", code)
}

pub fn compile(cfg: &str, ctx: &S) {
   let helpers = parse_file("stdlib/untyped.lm");
   let ctx = kv_merge(&helpers, &ctx);
   let mut fragments = HashMap::new();
   for (k,v) in kv_iter(&ctx) {
      let k = k.to_string();
      //if is helper, ignore
      if k.starts_with("::") {}
      //if is symbol, reduce and compile
      else {
         let block = ctx_eval(&ctx, &app(
            variable("::safe-compile-expression"),
            app(
               v,
               typ("Block"),
            )
         ));
         fragments.insert( k, block );
      }
   }
   let fragments = kv_s(&fragments);
   let program = ctx_eval(&ctx, &app(
      variable("::safe-compile-program"),
      app(
         fragments,
         typ("Program"),
      )
   ));
   assemble(cfg, &program);
}
