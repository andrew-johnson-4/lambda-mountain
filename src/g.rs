/*

Copyright 2023 - Andrew Johnson

This code and all related intellectual property is available under the terms of
the attached permissive MIT license. This license is intended only to protect
the future development of the project while otherwise allowing people to use
the code and IP as they would like. Please, just be nice.

G: A Basic Codegen

*/

use crate::*;
use std::process::Command;
use std::fs::File;
use std::io::Write;

fn flatten(output: &mut String, input: &S) {
   if is_cons(input) {
      flatten( output, &head(input) );
      flatten( output, &tail(input) );
   } else if is_atom(input) {
      let l = input.to_string();
      if l == "literal" || l == "variable" || l == "app" {}
      else if l=="\\t" { output.push('\t'); }
      else if l=="\\n" { output.push('\n'); }
      else if l=="(" { output.push('('); }
      else if l==")" { output.push(')'); }
      else if l=="\"" { output.push('"'); }
      else {
         output.push_str( &l );
         output.push( ' ' );
      }
   }
}

fn assemble(cfg: &str, program: &S) {
   let mut code = String::new();
   flatten( &mut code, program );
   if cfg.ends_with(".s") {
      let mut file = File::create(cfg).expect("Could not create file in Term::compile");
      file.write_all(code.as_bytes()).expect("Could not write to file in Term::compile");
   } else {
      let tmp_o = format!("tmp.{}.{}.o",std::process::id(), std::thread::current().id().as_u64() );
      let tmp_s = format!("tmp.{}.{}.s",std::process::id(), std::thread::current().id().as_u64() );
      let mut file = File::create(&tmp_s).expect("Could not create file in Term::compile");
      file.write_all(code.as_bytes()).expect("Could not write to file in Term::compile");

      Command::new("as")
              .arg(&tmp_s)
              .arg("-o")
              .arg(&tmp_o)
              .spawn()
              .expect("Could not run assembler in g::assemble")
              .wait()
              .expect("Could not wait for assembler in g::assemble");

      Command::new("ld")
              .arg("-s")
              .arg("-o")
              .arg(cfg)
              .arg(&tmp_o)
              .spawn()
              .expect("Could not run linker in g::assemble")
              .wait()
              .expect("Could not wait for linker in g::assemble");
   }
}

fn label_case(s: &str) -> String {
   format!("{}", s.replace("-","_"))
}

fn compile_expr(ctx: &S, e: &S) -> S {
   unimplemented!("compile_expr: {}", e)
}

fn compile_symbol(ctx: &S, e: &S) -> S {
   unimplemented!("compile_symbol: {}", e)
}

pub fn compile(cfg: &str, main_ctx: &S) {
   let helpers_ctx = parse_file("stdlib/helpers.lm");
   let mut program = ctx_eval_soft(&helpers_ctx, &app(
      variable("::safe-compile-program"),
      app(
         app( variable("main"), nil() ),
         typ("Program"),
      )
   ));
   let prelude_ctx = parse_file("stdlib/prelude.lm");
   for (k,v) in kv_iter(&prelude_ctx) {
      program = app(
         program,
         compile_symbol(&prelude_ctx, &ctx_eval_soft(&helpers_ctx, &v) ),
      );
   }
   for (k,v) in kv_iter(&main_ctx) {
      let k = k.to_string();
      program = app(
         program,
         app(
            variable(&format!("\n{}:\n",&label_case(&k))),
            compile_symbol(&main_ctx, &ctx_eval_soft(&helpers_ctx, &v) ),
         ),
      );
   }
   //at this point the program should be in normal form
   //so it is safe to just dump it to a file and finish
   assemble(cfg, &program);
}
