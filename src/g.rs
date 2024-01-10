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
      if !(output.ends_with(" ") ||
           output.ends_with("\t") ||
           output.ends_with("\n")) {
         output.push(' ');
      }
      flatten( output, &tail(input) );
   } else if is_atom(input) {
      let l = input.to_string();
      if l == "literal" || l == "variable" || l == "app" {}
      else if l=="\\t" { output.push('\t'); }
      else if l=="\\n" { output.push('\n'); }
      else if l=="(" { output.push('('); }
      else if l==")" { output.push(')'); }
      else if l==r#""\lparen""# { output.push_str(r#""(""#); }
      else if l==r#""\rparen""# { output.push_str(r#"")""#); }
      else if l==r#""\space""# { output.push_str(r#"" ""#); }
      else if l==r#""\nil""# { output.push_str(r#""()""#); }
      else if l=="$" { output.push('$'); }
      else if l==r#"\lparen"# { output.push_str(r#"("#); }
      else if l==r#"\rparen"# { output.push_str(r#")"#); }
      else if l==r#"\space"# { output.push_str(r#" "#); }
      else if l=="\"" { output.push('"'); }
      else {
         output.push_str( &l );
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
      let tmp_o = format!("tmp.o");
      let tmp_s = format!("tmp.s");
      let mut file = File::create(&tmp_s).expect("Could not create file in Term::compile");
      file.write_all(code.as_bytes()).expect("Could not write to file in Term::compile");

      let output = Command::new("as")
              .arg(&tmp_s)
              .arg("-o")
              .arg(&tmp_o)
              .spawn()
              .expect("Could not run assembler in g::assemble")
              .wait_with_output()
              .expect("Could not wait for assembler in g::assemble");
      if !output.status.success() {
         let err = String::from_utf8_lossy(&output.stderr).to_string();
         panic!("{}", err)
      }

      let output = Command::new("ld")
              .arg("-o")
              .arg(cfg)
              .arg(&tmp_o)
              .spawn()
              .expect("Could not run linker in g::assemble")
              .wait_with_output()
              .expect("Could not wait for linker in g::assemble");
      if !output.status.success() {
         let err = String::from_utf8_lossy(&output.stderr).to_string();
         panic!("{}", err)
      }
   }
}

const OPERATORS: [(&str,&str); 11] = [
   ("==", "equal"),
   ("!=", "inequal"),
   ("+", "plus"),
   ("-", "minus"),
   ("/", "div"),

   ("*", "mul"),
   ("%", "mod"),
   ("not", "not"),
   ("head", "head"),
   ("tail", "tail"),

   ("print-s", "print_s"),
];

fn label_case(s: &str) -> String {
   for (k,v) in OPERATORS {
   if s==k {
      return v.to_string();
   }}
   format!("{}", s.replace("-","_"))
}

static mut UUID_COUNTER: usize = 0;
fn uuid() -> String {
   let id = unsafe { UUID_COUNTER += 1; UUID_COUNTER };
   format!("_uuid_{}", id)
}

//returns (prog, data)
fn yield_atom(helpers_ctx: &S, program_ctx: &S, s: &str, offset: i64) -> (S,S,S,i64) {
   let id = uuid();
   (
      ctx_eval_soft(helpers_ctx, &app( variable("::yield-atom"), variable(&id) )),
      variable(&format!("\n{}:\n\t.ascii \"{}\"\n\t.zero 1\n", id, s)),
      program_ctx.clone(),
      offset,
   )
}

fn is_free(program_ctx: &S, s: &str) -> bool {
   for (k,_v) in OPERATORS {
   if s==k {
      return false;
   }}
   for (k,_v) in kv_iter(program_ctx) {
   let k = k.to_string();
   if s==k {
      return false;
   }}
   true
}

fn is_local(program_ctx: &S, s: &str) -> String {
   for (k,v) in kv_iter(program_ctx) {
   let k = k.to_string();
   if s==k {
      if head(&v).to_string() == "local" {
         return tail(&v).to_string();
      } else {
         return "".to_string();
      }
   }}
   "".to_string()
}

//returns (push program, pop program, new program_ctx, new offset)
fn declare_local(helpers_ctx: &S, program_ctx: &S, vname: &S, offset: i64) -> (S,S,S,i64) {
   let push_this = ctx_eval_soft(helpers_ctx, &variable("::push-this"));
   let unpush_this = ctx_eval_soft(helpers_ctx, &variable("::unpush-this"));
   let refer = local(&format!(
      "\tmov {}(%rbp), %r12\n \
       \tmov {}(%rbp), %r13\n \
       \tmov {}(%rbp), %r14\n \
       \tmov {}(%rbp), %r15\n",
      -offset*32 - 8,
      -offset*32 - 16,
      -offset*32 - 24,
      -offset*32 - 32,
   ));
   let program_ctx = kv_add( program_ctx, &vname, &refer );
   (push_this, unpush_this, program_ctx, offset+1)
}

//returns (push program, pop program, new program_ctx, new offset)
fn destructure_args(helpers_ctx: &S, program_ctx: &S, e: &S, offset: i64) -> (S,S,S,i64) {
   if is_nil(e) {
      ( s_nil(), s_nil(), program_ctx.clone(), offset )
   } else if head(&e).to_string()=="variable" {
      declare_local(helpers_ctx, program_ctx, &tail(&e), offset)
   } else if head(&e).to_string()=="app" {
      let arg_head = head(&tail(&e));
      let arg_tail = tail(&tail(&e));
      let store_this = ctx_eval_soft(helpers_ctx, &variable("::shadow-this"));
      let restore_this = ctx_eval_soft(helpers_ctx, &variable("::unshadow-this"));
      let (load_head,_,_,_) = compile_expr(helpers_ctx, program_ctx, &app(variable("head"),variable("$_")), offset );
      let (load_tail,_,_,_) = compile_expr(helpers_ctx, program_ctx, &app(variable("tail"),variable("$_")), offset );
      let (push_tail, pop_tail, program_ctx, offset) = destructure_args(helpers_ctx, program_ctx, &arg_tail, offset);
      let (push_head, pop_head, program_ctx, offset) = destructure_args(helpers_ctx, &program_ctx, &arg_head, offset);
      let prog = s_cons(store_this, load_tail);
      let prog = s_cons(prog, push_tail);
      let prog = s_cons(prog, restore_this);
      let prog = s_cons(prog, load_head);
      let prog = s_cons(prog, push_head);
      let unprog = s_cons(pop_head, pop_tail);
      (prog, unprog, program_ctx, offset)
   } else {
      panic!("Unexpected lhs in destructure_args: {}", e)
   }
}

//returns (program, data, new program_ctx, new offset)
fn compile_expr(helpers_ctx: &S, program_ctx: &S, e: &S, offset: i64) -> (S,S,S,i64) {
   let e = ctx_eval_soft(helpers_ctx, e);
   if head(&e).to_string() == "app" {
      let fx = tail(&e);
      let f = head(&fx);
      let x = tail(&fx);
      if head(&f).to_string() == "variable" &&
         tail(&f).to_string() == "local" &&
         head(&x).to_string() == "variable" {
         let lname = tail(&x).to_string();
         unimplemented!("declare local: {}", lname)
      } else if (head(&f).to_string() == "variable" ||
         head(&f).to_string() == "literal") &&
         !is_free(program_ctx, &tail(&f).to_string()) {
         let (xprog,xdata,program_ctx,offset) = compile_expr(helpers_ctx, program_ctx, &x, offset);
         let f_name = variable(&label_case( &tail(&f).to_string() ));
         let prog = s_cons( xprog , s_cons( s_cons( variable("\tcall"), f_name ), variable("\n") ));
         (prog, xdata, program_ctx.clone(), offset)
      } else {
         let (fprog,fdata,program_ctx,offset) = compile_expr(helpers_ctx, program_ctx, &f, offset);
         let (xprog,xdata,program_ctx,offset) = compile_expr(helpers_ctx, &program_ctx, &x, offset);
         let prog = ctx_eval_soft(helpers_ctx, &app(
            variable("::yield-cons"),
            app( fprog, xprog )
         ));
         let data = app(
            fdata,
            xdata,
         );
         (prog, data, program_ctx, offset)
      }
   } else if head(&e).to_string() == "variable" &&
             tail(&e).to_string() == "$_" {
      // $_ is a noop expression and colloquially refers to 'this' expression
      ( s_nil(), s_nil(), program_ctx.clone(), offset )
   } else if head(&e).to_string() == "variable" {
      let vname = tail(&e).to_string();
      let local = is_local(program_ctx, &vname);
      if local == "" {
         yield_atom(helpers_ctx, program_ctx, &vname, offset)
      } else {
         ( s_atom(&local), s_nil(), program_ctx.clone(), offset )
      }
   } else if head(&e).to_string() == "literal" {
      yield_atom(helpers_ctx, program_ctx, &tail(&e).to_string(), offset )
   } else if head(&e).to_string() == "lambda" {
      let args = head(&tail(&e));
      let body = tail(&tail(&e));
      let (push_prog,pop_prog,program_ctx,offset) = destructure_args(helpers_ctx, program_ctx, &args, 0);
      let (eprog,edata,program_ctx,_) = compile_expr(helpers_ctx, &program_ctx, &body, offset);
      let enter = ctx_eval_soft(helpers_ctx, &variable("::enter-function"));
      let leave = ctx_eval_soft(helpers_ctx, &variable("::leave-function"));
      let prog = enter;
      let prog = s_cons( prog, push_prog );
      let prog = s_cons( prog, eprog );
      let prog = s_cons( prog, pop_prog );
      let prog = s_cons( prog, leave );
      //TODO put locals into program_ctx
      //TODO compile body expression
      //TODO pop locals
      //don't forget to ret...
      ( prog, edata, program_ctx, offset )
   } else if is_nil(&e) {
      (
         ctx_eval_soft(helpers_ctx, &variable("::yield-nil")),
         nil(),
         program_ctx.clone(),
         offset,
      )
   } else {
      panic!("compile_expr unexpected term: {}", e);
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
   let mut offset = 0;
   let mut program_ctx = main_ctx.clone();
   for (k,v) in kv_iter(&main_ctx) {
      let k = k.to_string();
      let (vprog,vdata,pc,off) = compile_expr(&helpers_ctx, &program_ctx, &v, offset);
      program_ctx = pc;
      offset = off;
      raw_program = app(
         raw_program,
         app(
            variable(&format!("\n{}:\n",label_case(&k))),
            vprog,
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
         vdata,
      );
   }
   let program = compile_program(&helpers_ctx, &raw_program, &raw_data);
   assemble(cfg, &program);
}
