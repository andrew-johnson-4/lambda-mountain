/*

Copyright 2023 - Andrew Johnson

This code and all related intellectual property is available under the terms of
the attached permissive MIT license. This license is intended only to protect
the future development of the project while otherwise allowing people to use
the code and IP as they would like. Please, just be nice.

G: A Basic Codegen

*/

use crate::*;
use punc::*;

pub fn compile(_cfg: &str, ctx: &S) {
  let helpers = parse_file("stdlib/untyped.lm");
  let ctx = kv_merge(&helpers, &ctx);
  for (k,v) in kv_iter(&ctx) {
     println!("g::compile: {} := {}", k, v);
     //try to use LM bootstrap mostly
     //let e = safe_compile_expression(&v, &typ("Block"));
  }
  unimplemented!("g::compile")
  /*
  TODO: fix prelude and verify output before sending to PunC

  let t = compile_expression(s);

  let mut label_t = punc!( label nil_as_string (.asciz "()") (.zero 1) );
  for label in t.labels {
     label_t = punc!( {label} {label_t} );
  }

  let program = punc!(
      (.global _start)
      (.text)
      (label _start
         { t.term }
         (call print)
         (mov @60 %rax)
         (xor %rdi %rdi)
         (syscall)
      )
      ({ include!("../stdlib/prelude.rs") })
      ({label_t})
   );
   println!("program {}", program.to_string());
   program.compile(cfg);
   */
}
