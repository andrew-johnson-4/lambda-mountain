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
   assert_eq!( compile_and_run("tests/lm/123.lm"), "123" );
}

#[test]
fn cli_nil() {
   assert_eq!( compile_and_run("tests/lm/nil.lm"), "()" );
}

#[test]
fn cli_cons() {
   assert_eq!( compile_and_run("tests/lm/cons.lm"), "((123 ()) 456)" );
}

#[test]
fn cli_hello_world() {
   let exit = Command::new("lambda_mountain")
                      .arg("-o")
                      .arg("hello_world")
                      .arg("tests/lm/hello_world.lm")
                      .spawn()
                      .expect("failed to execute process")
                      .wait()
                      .expect("failed to wait for process");
   assert!(exit.success());
   let output = Command::new("./hello_world")
                            .stdout(std::process::Stdio::piped())
                            .spawn()
                            .expect("failed to execute process")
                            .wait_with_output()
                            .expect("failed to wait for process")
                            .stdout;
   let output = String::from_utf8_lossy(&output).to_string();
   assert_eq!( output, "hello_world" );
}

#[test]
fn cli_head() {
   assert_eq!( compile_and_run("tests/lm/head.lm"), "123" );
}

#[test]
fn cli_tail() {
   assert_eq!( compile_and_run("tests/lm/tail.lm"), "123" );
}

