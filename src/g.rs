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
      else if l=="$" { output.push('$'); }
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

static mut UUID_COUNTER: usize = 0;
fn uuid() -> String {
   let id = unsafe { UUID_COUNTER += 1; UUID_COUNTER };
   format!("_uuid_{}", id)
}

fn yield_atom(helpers_ctx: &S, s: &str) -> S {
   let id = uuid();
   s_cons(
      ctx_eval_soft(helpers_ctx, &app( variable("::yield-atom"), variable(&id) )),
      variable(&format!("{}:\n\t.ascii \"{}\"\n\t.zero 1\n", id, s)),
   )
}

fn compile_expr(helpers_ctx: &S, program_ctx: &S, e: &S) -> S {
   if head(e).to_string() == "lambda" {
      unimplemented!("compile_expr: {}", e);
   } else if head(e).to_string() == "app" {
      let fx = tail(e);
      let f = head(&fx);
      let x = tail(&fx);
      let xpg = compile_expr(helpers_ctx, program_ctx, &x);
      assert_eq!( head(&f).to_string(), "variable" );
      let f_name = label_case( &tail(&f).to_string() );
      let call = variable( &format!("\tcall {}\n", f_name) );
      let prog = app(
         head(&xpg),
         call
      );
      s_cons(prog, tail(&xpg))
   } else if head(e).to_string() == "variable" {
      yield_atom(helpers_ctx, &tail(e).to_string() )
   } else if head(e).to_string() == "literal" {
      yield_atom(helpers_ctx, &tail(e).to_string() )
   } else if is_nil(e) {
      s_cons(
         ctx_eval_soft(helpers_ctx, &variable("::yield-nil")),
         nil(),
      )
   } else {
      unimplemented!("compile_expr: {}", e);
   }
}

fn compile_program(helpers_ctx: &S, raw_program: &S, raw_data: &S) -> S {
   let head = ctx_eval_soft(&helpers_ctx, &variable("::program-header"));
   let head = app( head, raw_program.clone() );
   let data = ctx_eval_soft(&helpers_ctx, &variable("::data-header"));
   let data = app( data, raw_data.clone() );
   app(
      head,
      data,
   )
}

pub fn compile(cfg: &str, main_ctx: &S) {
   let helpers_ctx = parse_file("stdlib/helpers.lm");
   let prelude_ctx = parse_file("stdlib/prelude.lm");
   let mut raw_program = nil();
   let mut raw_data = nil();
   for (k,v) in kv_iter(&prelude_ctx) {
      let k = k.to_string();
      if k == ".data" {
         raw_data = app(
            raw_data,
            ctx_eval_soft(&helpers_ctx, &v),
         );
      } else if k == ".text" {
         raw_program = app(
            raw_program,
            ctx_eval_soft(&helpers_ctx, &v),
         );
      } else {
         panic!("unexpected prelude symbol: {}", k);
      }
   }
   for (k,v) in kv_iter(&main_ctx) {
      let k = k.to_string();
      let v = compile_expr(&helpers_ctx, &main_ctx, &v);
      raw_program = app(
         raw_program,
         app(
            variable(&format!("{}:\n",label_case(&k))),
            head(&v),
         ),
      );
      if k == "main" {
         raw_program = app(
            raw_program,
            ctx_eval_soft(&helpers_ctx, &variable("::exit-cleanup")),
         );
      }
      raw_data = app(
         raw_data,
         tail(&v),
      );
   }
   let program = compile_program(&helpers_ctx, &raw_program, &raw_data);
   assemble(cfg, &program);
}
