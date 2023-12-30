#![feature(thread_id_value)]

use std::process::Command;

fn compile_and_run(fp: &str) -> String {
   Command::new("lambda_mountain")
           .arg(fp)
           .spawn()
           .expect("failed to execute process")
           .wait()
           .expect("failed to wait for process");
   let output = Command::new("./a.out")
                            .stdout(std::process::Stdio::piped())
                            .spawn()
                            .expect("failed to execute process")
                            .wait_with_output()
                            .expect("failed to wait for process")
                            .stdout;
   String::from_utf8_lossy(&output).to_string()
}

#[test]
fn cli_123() {
   assert_eq!( compile_and_run("tests/123.lm"), "123" );
}
