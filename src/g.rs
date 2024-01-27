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
      if l=="lambda" || l=="literal" || l=="variable" || l=="app" || l=="local" || l=="type" {}
      else if l == "\\o" {
         output.push_str("#"); 
      } else if l == "\\[" {
         output.push_str("("); 
      } else if l == "\\]" {
         output.push_str(")"); 
      } else if l == "\\s" {
         output.push_str(" "); 
      } else if l == "\\l" {
         output.push_str("λ"); 
      } else if l == "\\:" {
         output.push_str(";"); 
      } else if l == "\\t" {
         output.push_str("\t"); 
      } else if l == "\\n" {
         output.push_str("\n"); 
      } else {
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

const OPERATORS: [(&str,&str); 21] = [
   ("eq", "eq"),
   ("not", "not"),
   ("inc", "inc"),
   ("dec", "dec"),
   ("add", "add"),
   ("mul", "mul"),
   ("div", "div"),
   ("mod", "mod"),
   ("inv", "inv"),
   ("is-neg", "is_neg"),

   ("head", "head"),
   ("tail", "tail"),
   ("dump-i", "dump_i"),
   ("print-s", "print_s"),
   ("print-i", "print_i"),
   ("print-p", "print_p"),
   ("print-d", "print_d"),
   ("clone-rope", "clone_rope"),
   ("write-file", "write_file"),
   ("load-file", "load_file"),

   ("digit", "digit"),
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

//returns (frame program, expression program, unframe program, text, data, new program_ctx, new offset)
fn yield_atom(_helpers_ctx: &S, program_ctx: &S, s: &str, offset: i64) -> (S,S,S,S,S,S,i64) {
   let s = s.replace("\n",r#"\n"#);
   let s = s.replace("\\\\","[ESCAPE]");
   let s = s.replace(r#"""#,r#"\""#);
   let s = s.replace("\\s"," ");
   let s = s.replace("\\:",";");
   let s = s.replace("\\,",".");
   let s = s.replace("\\o","#");
   let s = s.replace("\\l","λ");
   let s = s.replace("\\[","(");
   let s = s.replace("\\]",")");
   let s = s.replace("[ESCAPE]","\\\\");
   let id = uuid();
   let prog = s_nil();
   let prog = s_cons(prog, s_atom(&format!("\tmov ${}, %r12\n",id)));
   let prog = s_cons(prog, s_atom("\tmov $0, %r13\n"));
   let prog = s_cons(prog, s_atom("\tmov $0, %r14\n"));
   let prog = s_cons(prog, s_atom("\tmov $0, %r15\n"));
   (
      s_nil(),
      prog,
      s_nil(),
      s_nil(),
      literal(&format!("{}:\n\t.ascii \"{}\"\n\t.zero 1\n", id, s)),
      program_ctx.clone(),
      offset,
   )
}

//returns (frame program, expression program, unframe program, text, data, new program_ctx, new offset)
fn declare_local(helpers_ctx: &S, program_ctx: &S, vname: &S, offset: i64) -> (S,S,S,S,S,S,i64) {
  
   let frame_this = ctx_eval_soft(helpers_ctx, &variable("::push-zero"));
   let unframe_this = ctx_eval_soft(helpers_ctx, &variable("::unpush-this"));
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
   let assign = local(&format!(
      "\tmov %r12, {}(%rbp)\n \
       \tmov %r13, {}(%rbp)\n \
       \tmov %r14, {}(%rbp)\n \
       \tmov %r15, {}(%rbp)\n",
      -offset*32 - 8,
      -offset*32 - 16,
      -offset*32 - 24,
      -offset*32 - 32,
   ));
   let set_this = assign.clone();
   let assign_vname = s_atom(&format!("set {}",vname));
   let program_ctx = kv_add( program_ctx, &vname, &refer );
   let program_ctx = kv_add( &program_ctx, &assign_vname, &assign );
   (frame_this, set_this, unframe_this, s_nil(), s_nil(), program_ctx, offset+1)
}

//returns (frame program, expression program, unframe program, text, data, new program_ctx, new offset)
fn destructure_args(helpers_ctx: &S, program_ctx: &S, e: &S, offset: i64) -> (S,S,S,S,S,S,i64) {
   if is_nil(e) {
      ( s_nil(), s_nil(), s_nil(), s_nil(), s_nil(), program_ctx.clone(), offset )
   } else if head(&e).to_string()=="variable" {
      declare_local(helpers_ctx, program_ctx, &tail(&e), offset)
   } else if head(&e).to_string()=="app" {
      let arg_head = head(&tail(&e));
      let arg_tail = tail(&tail(&e));
      let store_this = ctx_eval_soft(helpers_ctx, &variable("::shadow-this"));
      let restore_this = ctx_eval_soft(helpers_ctx, &variable("::unshadow-this"));
      let (_frame,load_head,_unframe,_text,_data,_ctx,_) = compile_expr(helpers_ctx, program_ctx, &app(variable("head"),variable("$_")), offset );
      let (_frame,load_tail,_unframe,_text,_data,_ctx,_) = compile_expr(helpers_ctx, program_ctx, &app(variable("tail"),variable("$_")), offset );
      let (frame_tail, prog_tail, unframe_tail, _text, _data, program_ctx, offset) = destructure_args(helpers_ctx, program_ctx, &arg_tail, offset);
      let (frame_head, prog_head, unframe_head, _text, _data, program_ctx, offset) = destructure_args(helpers_ctx, &program_ctx, &arg_head, offset);
      let prog = s_cons(store_this, load_tail);
      let prog = s_cons(prog, prog_tail);
      let prog = s_cons(prog, restore_this);
      let prog = s_cons(prog, load_head);
      let prog = s_cons(prog, prog_head);
      let frame = s_cons(frame_head, frame_tail);
      let unframe = s_cons(unframe_head, unframe_tail);
      (frame, prog, unframe, s_nil(), s_nil(), program_ctx, offset)
   } else {
      panic!("Unexpected lhs in destructure_args: {}", e)
   }
}

//returns (frame program, expression program, unframe program, text, data, new program_ctx, new offset)
fn destructure_pattern_lhs(helpers_ctx: &S, program_ctx: &S, p: &S, offset: i64) -> (S,S,S,S,S,S,i64) {
   if head(&p).to_string() == "variable" &&
      tail(&p).to_string() == "_" {
      let set_rsi = s_atom("\tmov $1, %rsi\n");
      ( s_nil(), set_rsi, s_nil(), s_nil(), s_nil(), program_ctx.clone(), offset )
   } else if head(&p).to_string()=="variable" {
      let (lframe,lprog,lunframe,ltext,ldata,program_ctx,offset) = declare_local(helpers_ctx, program_ctx, &tail(&p), offset);
      let set_rsi = s_atom("\tmov $1, %rsi\n");
      let prog = s_cons(lprog, set_rsi);
      (lframe, prog, lunframe, ltext, ldata, program_ctx, offset)
   } else if head(&p).to_string()=="literal" {
      let label_skip = uuid();
      let prog = ctx_eval_soft(helpers_ctx, &variable("::shadow-this"));
      let prog = s_cons(prog, s_atom("\tmov %r12, %rax\n"));
      let (_aframe,aprog,_aunframe,atext,adata,_program_ctx,_offset) = yield_atom(helpers_ctx, program_ctx, &tail(&p).to_string(), offset);
      let prog = s_cons(prog, aprog);
      let prog = s_cons(prog, s_atom("\tmov %r12, %rbx\n"));
      let prog = s_cons(prog, s_atom("\tcall _streq\n"));
      let prog = s_cons(prog, s_atom("\tmov %r12, %rdi\n"));
      let prog = s_cons(prog, ctx_eval_soft(helpers_ctx, &variable("::unshadow-this")));
      let prog = s_cons(prog, s_atom("\tcmp $0, %rdi\n"));
      let prog = s_cons(prog, s_atom(&format!("\tje {}\n",label_skip)));
      let prog = s_cons(prog, s_atom("\tmov $1, %rsi\n"));
      let prog = s_cons(prog, s_atom(&format!("{}:\n",label_skip)));
      ( s_nil(), prog, s_nil(), atext, adata, program_ctx.clone(), offset )
   } else if head(&p).to_string()=="app" {
      let label_skip = uuid();
      let l = head(&tail(&p));
      let r = tail(&tail(&p));
      let (lframe,lprog,lunframe,ltext,ldata,program_ctx,offset) = destructure_pattern_lhs(helpers_ctx, program_ctx, &l, offset);
      let (rframe,rprog,runframe,rtext,rdata,program_ctx,offset) = destructure_pattern_lhs(helpers_ctx, &program_ctx, &r, offset);
      let prog = s_atom("\tmov $0, %rsi\n");
      let prog = s_cons(prog, ctx_eval_soft(helpers_ctx, &variable("::push-this")));
      let prog = s_cons(prog, s_atom("\tcmp $0, %r13\n"));
      let prog = s_cons(prog, s_atom(&format!("\tje {}\n",label_skip)));
      let prog = s_cons(prog, ctx_eval_soft(helpers_ctx, &variable("::head")));
      let prog = s_cons(prog, lprog);
      let prog = s_cons(prog, ctx_eval_soft(helpers_ctx, &variable("::pop-this")));
      let prog = s_cons(prog, ctx_eval_soft(helpers_ctx, &variable("::push-this")));
      let prog = s_cons(prog, s_atom("\tcmp $0, %rsi\n"));
      let prog = s_cons(prog, s_atom(&format!("\tje {}\n",label_skip)));
      let prog = s_cons(prog, s_atom("\tmov $0, %rsi\n"));
      let prog = s_cons(prog, s_atom("\tcmp $0, %r14\n"));
      let prog = s_cons(prog, s_atom(&format!("\tje {}\n",label_skip)));
      let prog = s_cons(prog, ctx_eval_soft(helpers_ctx, &variable("::tail")));
      let prog = s_cons(prog, rprog);
      let prog = s_cons(prog, s_atom(&format!("{}:\n",label_skip)));
      let prog = s_cons(prog, ctx_eval_soft(helpers_ctx, &variable("::pop-this")));
      (
         s_cons(lframe,rframe),
         prog,
         s_cons(lunframe,runframe),
         s_cons(ltext,rtext),
         s_cons(ldata,rdata),
         program_ctx,
         offset
      )
   } else if is_nil(&p) {
      let label_skip = uuid();
      let prog = s_atom("\tcmp $0, %r12\n");
      let prog = s_cons(prog, s_atom(&format!("\tjne {}\n",label_skip)));
      let prog = s_cons(prog, s_atom("\tcmp $0, %r13\n"));
      let prog = s_cons(prog, s_atom(&format!("\tjne {}\n",label_skip)));
      let prog = s_cons(prog, s_atom("\tcmp $0, %r14\n"));
      let prog = s_cons(prog, s_atom(&format!("\tjne {}\n",label_skip)));
      let prog = s_cons(prog, s_atom("\tcmp $0, %r15\n"));
      let prog = s_cons(prog, s_atom(&format!("\tjne {}\n",label_skip)));
      let prog = s_cons(prog, s_atom("\tmov $1, %rsi\n"));
      let prog = s_cons(prog, s_atom(&format!("{}:\n",label_skip)));
      ( s_nil(), prog, s_nil(), s_nil(), s_nil(), program_ctx.clone(), offset )
   } else {
      panic!("unexpected pattern lhs: {}", p)
   }
}

//returns (frame program, expression program, unframe program, text, data, new program_ctx, new offset)
fn yield_patterns(helpers_ctx: &S, program_ctx: &S, p: &S, offset: i64) -> (S,S,S,S,S,S,i64) {
   if is_nil(p) {
      let clear_rsi = s_atom("\tmov $0, %rsi\n");
      ( s_nil(), clear_rsi, s_nil(), s_nil(), s_nil(), program_ctx.clone(), offset )
   } else if head(&p).to_string()=="app" &&
             head(&tail(&tail(&p))).to_string()=="app" {
      let prev = head(&tail(&p));
      let lr = tail(&tail(&tail(&p)));
      let lhs = head(&lr);
      let rhs = tail(&lr);
      let (pframe,pprog,punframe,ptext,pdata,_inner_ctx,offset) = yield_patterns(helpers_ctx, program_ctx, &prev, offset);
      let (lframe,lprog,lunframe,ltext,ldata,inner_ctx,offset) = destructure_pattern_lhs(helpers_ctx, &program_ctx, &lhs, offset);
      let (rframe,rprog,runframe,rtext,rdata,_inner_ctx,offset) = compile_expr(helpers_ctx, &inner_ctx, &rhs, offset);
      let label_skip = uuid();
      let prog = pprog;
      let prog = s_cons(prog, s_atom(&format!("\tcmp $0, %rsi\n\tjne {}\n",label_skip)));
      let prog = s_cons(prog, lprog); //set %rsi to non-zero if success
      let prog = s_cons(prog, s_atom(&format!("\tcmp $0, %rsi\n\tje {}\n",label_skip)));
      let prog = s_cons(prog, rprog);
      let prog = s_cons(prog, s_atom("\tmov $1, %rsi\n"));
      let prog = s_cons(prog, s_atom(&format!("{}:\n",label_skip)));
      (
         s_cons(s_cons(pframe,lframe),rframe),
         prog,
         s_cons(s_cons(punframe,lunframe),runframe),
         s_cons(s_cons(ptext,ltext),rtext),
         s_cons(s_cons(pdata,ldata),rdata),
         program_ctx.clone(),
         offset
      )
   } else {
      panic!("invalid patterns case: {}", p)
   }
}

//returns (frame program, expression program, unframe program, text, data, new program_ctx, new offset)
fn compile_expr(helpers_ctx: &S, program_ctx: &S, e: &S, offset: i64) -> (S,S,S,S,S,S,i64) {
   let e = ctx_eval_soft(helpers_ctx, e);
   if head(&e).to_string() == "app" {
      let fx = tail(&e);
      let f = head(&fx);
      let x = tail(&fx);
      if head(&f).to_string() == "variable" &&
         tail(&f).to_string() == "local" &&
         head(&x).to_string() == "variable" {
	 let (f,_p,u,t,d,pc,offset) = declare_local(helpers_ctx, program_ctx, &tail(&x), offset);
         ( f, s_nil(), u, t, d, pc, offset )
      } else if head(&f).to_string()=="app" &&
                head(&head(&tail(&f))).to_string() == "variable" &&
                tail(&head(&tail(&f))).to_string() == "set" &&
                head(&tail(&tail(&f))).to_string() == "variable" {
         let lname = tail(&tail(&tail(&f))).to_string();
         let local = is_local(program_ctx, &format!("set {}", lname));
         if local == "" {
            let (xframe,xprog,xunframe,xtext,xdata,program_ctx,offset) = compile_expr(helpers_ctx, program_ctx, &x, offset);
            let prog = s_cons(xprog, ctx_eval_soft(helpers_ctx, &app(variable("::set-global"), variable(&label_case(&lname)) )) );
            ( xframe, prog, xunframe, xtext, xdata, program_ctx.clone(), offset )
         } else {
            let (xframe,xprog,xunframe,xtext,xdata,program_ctx,offset) = compile_expr(helpers_ctx, program_ctx, &x, offset);
            ( xframe, s_cons(xprog, s_atom(&local)), xunframe, xtext, xdata, program_ctx.clone(), offset )
         }
      } else if head(&e).to_string()=="app" &&
                head(&head(&tail(&e))).to_string() == "app" &&
                head(&head(&tail(&head(&tail(&e))))).to_string() == "variable" &&
                tail(&head(&tail(&head(&tail(&e))))).to_string() == "foreach-atom" {
         let atom = tail(&tail(&head(&tail(&e)))); 
         let apply_expr = tail(&tail(&e));
         let foreach_label = s_atom(&uuid());
         let foreach_notcons = s_atom(&uuid());
         let foreach_ignore = s_atom(&uuid());
         let (aframe,aprog,aunframe,atext,adata,program_ctx,offset) = compile_expr(helpers_ctx, program_ctx, &atom, offset);
         let (eframe,eprog,eunframe,etext,edata,program_ctx,offset) = if head(&apply_expr).to_string()=="variable" {
            let apply_label = tail(&apply_expr);
            let apply_prog = s_atom(&format!("\tcall {}\n", label_case(&apply_label.to_string())));
            (s_nil(), apply_prog, s_nil(), s_nil(), s_nil(), program_ctx.clone(), offset)
         } else {
            compile_expr(helpers_ctx, &program_ctx, &apply_expr, offset)
         };
         let ftext = ctx_eval_soft(helpers_ctx, &app(variable("::foreach-atom"),app(app(app(foreach_label.clone(),foreach_notcons),foreach_ignore),eprog.clone())));
         let prog = s_cons( aprog, s_atom(&format!("\tcall {}\n",foreach_label)) );
         ( s_cons(aframe,eframe), prog, s_cons(aunframe,eunframe), s_cons(s_cons(atext,ftext),etext), s_cons(adata,edata), program_ctx, offset )
      } else if head(&e).to_string()=="app" &&
                head(&head(&tail(&e))).to_string() == "app" &&
                head(&head(&tail(&head(&tail(&e))))).to_string() == "variable" &&
                tail(&head(&tail(&head(&tail(&e))))).to_string() == "foreach-char" {
         let atom = tail(&tail(&head(&tail(&e)))); 
         let apply_expr = tail(&tail(&e));
         let foreach_head = s_atom(&uuid());
         let foreach_small = s_atom(&uuid());
         let foreach_end = s_atom(&uuid());
         let foreach_notcons = s_atom(&uuid());
         let foreach_data = s_atom(&uuid());
         let foreach_apply = s_atom(&uuid());
         let (aframe,prog,aunframe,atext,adata,program_ctx,offset) = compile_expr(helpers_ctx, program_ctx, &atom, offset);
         let (eframe,eprog,eunframe,etext,edata,program_ctx,offset) = if head(&apply_expr).to_string()=="variable" {
            let apply_label = tail(&apply_expr);
            let apply_prog = s_atom(&format!("\tcall {}\n", label_case(&apply_label.to_string())));
            (s_nil(), apply_prog, s_nil(), s_nil(), s_nil(), program_ctx.clone(), offset)
         } else {
            compile_expr(helpers_ctx, &program_ctx, &apply_expr, offset)
         };
         let ftext = ctx_eval_soft(helpers_ctx, &app(variable("::foreach-char"),
            app(app(app(app(
               app(app(foreach_data.clone(),foreach_head.clone()),foreach_small.clone())
           ,foreach_end),foreach_notcons),foreach_apply),eprog.clone())
         ));
         let fdata = ctx_eval_soft(helpers_ctx, &app(variable("::foreach-char-data"),foreach_data.clone()));
         let prog = s_cons( prog, s_atom(&format!("\tcall {}\n",foreach_head)) );
         ( s_cons(aframe,eframe), prog, s_cons(aunframe,eunframe), s_cons(s_cons(atext,ftext),etext), s_cons(s_cons(adata,fdata),edata), program_ctx, offset )
      } else if head(&e).to_string()=="app" &&
                head(&head(&tail(&e))).to_string() == "app" &&
                head(&head(&tail(&head(&tail(&e))))).to_string() == "variable" &&
                tail(&head(&tail(&head(&tail(&e))))).to_string() == "while" {
         let d = tail(&tail(&e));
         let c = tail(&tail(&head(&tail(&e))));
         let (c_f,c_p,c_u,c_t,c_d,program_ctx,offset) = compile_expr(helpers_ctx, program_ctx, &c, offset);
         let (d_f,d_p,d_u,d_t,d_d,program_ctx,offset) = compile_expr(helpers_ctx, &program_ctx, &d, offset);
         let label_w_start = uuid();
         let label_w_end = uuid();
         let prog = s_atom(&format!("{}:\n",label_w_start));
         let prog = s_cons( prog, c_p );
         let prog = s_cons( prog, s_atom("\tmov $0, %rax\n") );
         let prog = s_cons( prog, s_atom("\tadd %r12, %rax\n") );
         let prog = s_cons( prog, s_atom("\tadd %r13, %rax\n") );
         let prog = s_cons( prog, s_atom("\tadd %r14, %rax\n") );
         let prog = s_cons( prog, s_atom("\tadd %r15, %rax\n") );
         let prog = s_cons( prog, s_atom("\tcmp $0, %rax\n") );
         let prog = s_cons( prog, s_atom(&format!("\tje {}\n", label_w_end)) );
         let prog = s_cons( prog, d_p );
         let prog = s_cons( prog, s_atom(&format!("\tjmp {}\n", label_w_start)) );
         let prog = s_cons( prog, s_atom(&format!("{}:\n",label_w_end)) );
         (
            s_cons(c_f,d_f),
            prog,
            s_cons(c_u,d_u),
            s_cons(c_t,d_t),
            s_cons(c_d,d_d),
            program_ctx,
            offset
         )
      } else if head(&e).to_string()=="app" &&
                head(&head(&tail(&e))).to_string() == "app" &&
                head(&head(&tail(&head(&tail(&e))))).to_string() == "app" &&
                head(&head(&tail(&head(&tail(&head(&tail(&e))))))).to_string() == "variable" &&
                tail(&head(&tail(&head(&tail(&head(&tail(&e))))))).to_string() == "if" {         
         let f = tail(&tail(&e));
         let t = tail(&tail(&head(&tail(&e))));
         let c = tail(&tail(&head(&tail(&head(&tail(&e))))));
         let (c_f,c_p,c_u,c_t,c_d,program_ctx,offset) = compile_expr(helpers_ctx, program_ctx, &c, offset);
         let (t_f,t_p,t_u,t_t,t_d,program_ctx,offset) = compile_expr(helpers_ctx, &program_ctx, &t, offset);
         let (f_f,f_p,f_u,f_t,f_d,program_ctx,offset) = compile_expr(helpers_ctx, &program_ctx, &f, offset);
         let label_if_true = uuid();
         let label_if_end = uuid();
         let prog = c_p;
         let prog = s_cons( prog, s_atom(&format!("\tcmp $0, %r12\n\tjne {}\n", label_if_true)) );
         let prog = s_cons( prog, s_atom(&format!("\tcmp $0, %r13\n\tjne {}\n", label_if_true)) );
         let prog = s_cons( prog, s_atom(&format!("\tcmp $0, %r14\n\tjne {}\n", label_if_true)) );
         let prog = s_cons( prog, s_atom(&format!("\tcmp $0, %r15\n\tjne {}\n", label_if_true)) );
         let prog = s_cons( prog, f_p );
         let prog = s_cons( prog, s_atom(&format!("\tjmp {}\n", label_if_end)) );
         let prog = s_cons( prog, s_atom(&format!("{}:\n",label_if_true)) );
         let prog = s_cons( prog, t_p );
         let prog = s_cons( prog, s_atom(&format!("{}:\n",label_if_end)) );
         (
            s_cons(s_cons(c_f,t_f),f_f),
            prog,
            s_cons(s_cons(c_u,t_u),f_u),
            s_cons(s_cons(c_t,t_t),f_t),
            s_cons(s_cons(c_d,t_d),f_d),
            program_ctx,
            offset
         )
      } else if head(&e).to_string()=="app" &&
                head(&head(&tail(&e))).to_string() == "app" &&
                head(&head(&tail(&head(&tail(&e))))).to_string() == "variable" &&
                tail(&head(&tail(&head(&tail(&e))))).to_string() == "match" {
         let p = tail(&tail(&e));
         let c = tail(&tail(&head(&tail(&e))));
         let (cframe,cprog,cunframe,ctext,cdata,program_ctx,offset) = compile_expr(helpers_ctx, program_ctx, &c, offset);
         let (pframe,pprog,punframe,ptext,pdata,program_ctx,offset) = yield_patterns(helpers_ctx, &program_ctx, &p, offset);
         let label_skip = uuid();
         let prog = s_cons(cprog,pprog);
         let prog = s_cons(prog, s_atom(&format!("\tcmp $0, %rsi\n\tjne {}\n",label_skip)));
         let prog = s_cons(prog, ctx_eval_soft(helpers_ctx, &variable("::yield-nil")) );
         let prog = s_cons(prog, s_atom(&format!("{}:\n",label_skip)));
         ( s_cons(cframe,pframe), prog, s_cons(cunframe,punframe), s_cons(ctext,ptext), s_cons(cdata,pdata), program_ctx, offset )
      } else if (head(&f).to_string() == "variable" ||
         head(&f).to_string() == "literal") &&
         !is_free(program_ctx, &tail(&f).to_string()) &&
         is_local(program_ctx, &tail(&f).to_string())=="" {
         let (xframe,xprog,xunframe,xtext,xdata,program_ctx,offset) = compile_expr(helpers_ctx, program_ctx, &x, offset);
         let f_name = variable(&label_case( &tail(&f).to_string() ));
         let prog = s_cons( xprog , s_cons( s_cons( variable("\tcall "), f_name ), variable("\n") ));
         (xframe, prog, xunframe, xtext, xdata, program_ctx.clone(), offset)
      } else {
         let (fframe,fprog,funframe,ftext,fdata,program_ctx,offset) = compile_expr(helpers_ctx, program_ctx, &f, offset);
         let (xframe,xprog,xunframe,xtext,xdata,program_ctx,offset) = compile_expr(helpers_ctx, &program_ctx, &x, offset);
         let prog = ctx_eval_soft(helpers_ctx, &app(
            variable("::yield-cons"),
            app( fprog, xprog )
         ));
         (s_cons(fframe,xframe), prog, s_cons(funframe,xunframe), s_cons(ftext,xtext), s_cons(fdata,xdata), program_ctx, offset)
      }
   } else if head(&e).to_string() == "variable" &&
             tail(&e).to_string() == "argv" {
      // $_ is a noop expression and colloquially refers to 'this' expression
      let atext = ctx_eval_soft(helpers_ctx, &variable("::argv"));
      ( s_nil(), atext, s_nil(), s_nil(), s_nil(), program_ctx.clone(), offset )
   } else if head(&e).to_string() == "variable" &&
             tail(&e).to_string() == "$_" {
      // $_ is a noop expression and colloquially refers to 'this' expression
      ( s_nil(), s_nil(), s_nil(), s_nil(), s_nil(), program_ctx.clone(), offset )
   } else if head(&e).to_string() == "variable" {
      let vname = tail(&e).to_string();
      let local = is_local(program_ctx, &vname);
      if local == "" {
         let prog = ctx_eval_soft(helpers_ctx, &app(variable("::get-global"), variable(&vname)) );
         ( s_nil(), prog, s_nil(), s_nil(), s_nil(), program_ctx.clone(), offset )
      } else {
         ( s_nil(), s_atom(&local), s_nil(), s_nil(), s_nil(), program_ctx.clone(), offset )
      }
   } else if head(&e).to_string() == "literal" {
      yield_atom(helpers_ctx, program_ctx, &tail(&e).to_string(), offset )
   } else if head(&e).to_string() == "lambda" {
      let args = head(&tail(&e));
      let body = tail(&tail(&e));
      let (frame_args,prog_args,unframe_args,_text,_data,program_ctx,offset) = destructure_args(helpers_ctx, program_ctx, &args, 0);
      let (eframe,eprog,eunframe,etext,edata,program_ctx,_) = compile_expr(helpers_ctx, &program_ctx, &body, offset);
      let enter = ctx_eval_soft(helpers_ctx, &variable("::enter-function"));
      let leave = ctx_eval_soft(helpers_ctx, &variable("::leave-function"));
      let prog = enter;
      let prog = s_cons( prog, frame_args );
      let prog = s_cons( prog, eframe );
      let prog = s_cons( prog, prog_args );
      let prog = s_cons( prog, eprog );
      let prog = s_cons( prog, unframe_args );
      let prog = s_cons( prog, eunframe );
      let prog = s_cons( prog, leave );
      ( s_nil(), prog, s_nil(), etext, edata, program_ctx, offset )
   } else if is_nil(&e) {
      (
         s_nil(),
         ctx_eval_soft(helpers_ctx, &variable("::yield-nil")),
         s_nil(),
         s_nil(),
         s_nil(),
         program_ctx.clone(),
         offset,
      )
   } else {
      panic!("compile_expr unexpected term: {}", e);
   }
}

fn compile_program(helpers_ctx: &S, raw_program: &S, raw_data: &S) -> S {
   let head = ctx_eval_soft(&helpers_ctx, &variable("::program-header"));
   let head = s_cons( head, raw_program.clone() );
   let data = ctx_eval_soft(&helpers_ctx, &variable("::data-header"));
   let data = s_cons( data, raw_data.clone() );
   s_cons(
      head,
      data,
   )
}

pub fn compile(debug: bool, cfg: &str, main_ctx: &S) {
   let mut main_ctx = main_ctx.clone();
   let helpers_ctx = parse_file("stdlib/helpers.lm");
   let prelude_ctx = parse_file("stdlib/prelude.lm");
   let mut raw_program = nil();
   let mut raw_data = nil();
   for (k,v) in kv_iter(&prelude_ctx) {
      let ks = k.to_string();
      if ks == "::data" {
         raw_data = app(
            raw_data,
            ctx_eval_soft(&helpers_ctx, &v),
         );
      } else if ks == "::text" {
         raw_program = app(
            raw_program,
            ctx_eval_soft(&helpers_ctx, &v),
         );
      } else {
         main_ctx = kv_add(&main_ctx, &k, &v);
      }
   }
   let mut has_main = false;
   for (k,v) in kv_iter(&main_ctx) {
   if is_nil(&v) {
      let k = k.to_string();
      raw_data = s_cons(
         raw_data,
         s_atom(&format!("{}:\n\t.zero 32\n",label_case(&k))),
      );
   } else {
      let k = k.to_string();
      raw_program = s_cons(
         raw_program,
         s_atom(&format!("{}:\n",label_case(&k))),
      );
      if k == "main" {
         has_main = true;
         let start = ctx_eval_soft(&helpers_ctx, &variable("::before-main"));
         let enter = ctx_eval_soft(&helpers_ctx, &variable("::enter-function"));
         raw_program = s_cons( raw_program, start );
         raw_program = s_cons( raw_program, enter );
         let (vframe,vprog,vunframe,vtext,vdata,_pc,_offset) = compile_expr(&helpers_ctx, &main_ctx, &v, 0);
         raw_program = s_cons( raw_program, vframe );
         raw_program = s_cons( raw_program, vprog );
         raw_program = s_cons( raw_program, vunframe );
         raw_program = s_cons( raw_program, ctx_eval_soft(&helpers_ctx, &variable("::exit-cleanup")) );
         raw_program = s_cons( raw_program, vtext );
         raw_data = s_cons(raw_data,vdata);
      } else if k.starts_with("::") {
         //ignore completely
      } else if k.starts_with("_") {
         let mut buf = String::new();
         flatten(&mut buf, &v);
         let (vframe,vprog,vunframe,vtext,vdata,_pc,_offset) = yield_atom(&helpers_ctx, &main_ctx, &buf, 0);
         raw_program = s_cons( raw_program, vframe );
         raw_program = s_cons( raw_program, vprog );
         raw_program = s_cons( raw_program, vunframe );
         raw_program = s_cons( raw_program, vtext );
         raw_program = s_cons( raw_program, s_atom("\tret\n") );
         raw_data = s_cons(raw_data,vdata);
      } else {
         let (vframe,vprog,vunframe,vtext,vdata,_pc,_offset) = compile_expr(&helpers_ctx, &main_ctx, &v, 0);
         raw_program = s_cons( raw_program, vframe );
         raw_program = s_cons( raw_program, vprog );
         raw_program = s_cons( raw_program, vunframe );
         raw_program = s_cons( raw_program, vtext );
         raw_data = s_cons(raw_data,vdata);
      }
   }}
   if !has_main {
      raw_program = s_cons( raw_program, s_atom("main:\n") );
      raw_program = s_cons( raw_program, ctx_eval_soft(&helpers_ctx, &variable("::exit-cleanup")) );
   }
   let program = compile_program(&helpers_ctx, &raw_program, &raw_data);
   assemble(cfg, &program);
   if !debug {
      rm("tmp.s");
      rm("tmp.o");
   }
}

fn rm(p: &str) {
   if std::path::Path::new(p).is_file() {
      std::fs::remove_file(p).expect(&format!("Could not remove file: {}",p))
   }
   assert!( !std::path::Path::new(p).is_file() );
}
