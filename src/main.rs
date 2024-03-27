

enum CompileMode { Tokenize, Parse, Compile }

use lambda_mountain::*;

static mut CONFIG_MODE: CompileMode = CompileMode::Compile;

fn main() {
   let mut inputs = Vec::new();
   for arg in std::env::args() {
      if arg=="--tokenize" {unsafe{ CONFIG_MODE = CompileMode::Tokenize; }}
      else if arg=="--parse" {unsafe{ CONFIG_MODE = CompileMode::Parse; }}
      else if arg=="--compile" {unsafe{ CONFIG_MODE = CompileMode::Compile; }}
      else { inputs.push(arg); }
   }
}
