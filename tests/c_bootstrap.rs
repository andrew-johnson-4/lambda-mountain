#![feature(thread_id_value)]

use std::process::Command;
use lambda_mountain::*;

fn compile_and_run(s: &S) -> String {
   let a_out = format!("tmp.{}.{}.exe",std::process::id(),std::thread::current().id().as_u64());
   compile(&a_out, s);

   let output = Command::new(&format!("./{}",a_out))
                            .spawn()
                            .expect("failed to execute process")
                            .wait_with_output()
                            .expect("failed to wait for process")
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
