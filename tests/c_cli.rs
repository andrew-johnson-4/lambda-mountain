use std::process::Command;

fn compile_and_run(fp: &str) -> String {
   let exit = Command::new("lambda_mountain")
                      .arg(fp)
                      .spawn()
                      .expect("failed to execute process")
                      .wait()
                      .expect("failed to wait for process");
   assert!(exit.success());
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

#[test]
fn cli_nil() {
   assert_eq!( compile_and_run("tests/nil.lm"), "()" );
}

#[test]
fn cli_cons() {
   assert_eq!( compile_and_run("tests/cons.lm"), "(123 . (() . 456))" );
}
