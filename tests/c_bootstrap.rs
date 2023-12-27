use std::process::Command;
use lambda_mountain::*;

fn compile_and_run(s: &S) -> String {
   compile("a.out", s);

   let output = Command::new("./a.out")
                            .output()
                            .expect("failed to execute process")
                            .stdout;
   String::from_utf8_lossy(&output).to_string()
}

#[test]
fn nil() {
   assert_eq!( compile_and_run(&s_nil()), "" );
}

#[test]
fn hello_world() {
   assert_eq!( compile_and_run(&s_atom("hello world")), "hello world" );
}
