

enum CompileMode { Tokenize, Parse, Typecheck, Compile }

use lambda_mountain::*;

static mut CONFIG_MODE: CompileMode = CompileMode::Compile;
static mut CONFIG_STRICT: bool = true;

fn main() {
   let mut inputs = Vec::new();
   let mut target = "tmp.s".to_string();
   let mut set_target = false;
   for arg in std::env::args() {
      if set_target { target = arg.to_string(); }
      else if arg=="--tokenize" {unsafe{ CONFIG_MODE = CompileMode::Tokenize; }}
      else if arg=="--parse" {unsafe{ CONFIG_MODE = CompileMode::Parse; }}
      else if arg=="--typecheck" {unsafe{ CONFIG_MODE = CompileMode::Typecheck; }}
      else if arg=="--compile" {unsafe{ CONFIG_MODE = CompileMode::Compile; }}
      else if arg=="--strict" {unsafe{ CONFIG_STRICT = true; }}
      else if arg=="--gradual" {unsafe{ CONFIG_STRICT = false; }}
      else if arg=="-o" { set_target = true; }
      else { inputs.push(arg); }
   }

   for input in inputs { unsafe { match CONFIG_MODE {
      CompileMode::Compile => { parse_program(tokenize_file(&input)); }
      CompileMode::Parse => { parse_program(tokenize_file(&input)); }
      CompileMode::Typecheck => { parse_program(tokenize_file(&input)); }
      CompileMode::Tokenize => { tokenize_file(&input).print(); }
   }}}

   println!("output: {}", target);
}
