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
fn cli_cli() {
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
fn cli_yield() {
   assert_eq!( compile_and_run("tests/lm/nil.lm"), "()" );
   assert_eq!( compile_and_run("tests/lm/123.lm"), "123" );
   assert_eq!( compile_and_run("tests/lm/cons.lm"), "((123 ()) 456)" );
}

#[test]
fn cli_headtail() {
   assert_eq!( compile_and_run("tests/lm/head.lm"), "123" );
   assert_eq!( compile_and_run("tests/lm/head_is_nil.lm"), "()" );
   assert_eq!( compile_and_run("tests/lm/tail.lm"), "123" );
   assert_eq!( compile_and_run("tests/lm/tail_is_nil.lm"), "()" );
}

#[test]
fn cli_comparison() {
   assert_eq!( compile_and_run("tests/lm/atom_comparison_not.lm"), "True" );
   assert_eq!( compile_and_run("tests/lm/atom_comparison_notnot.lm"), "()" );
   assert_eq!( compile_and_run("tests/lm/atom_comparison_inequal.lm"), "()" );
   assert_eq!( compile_and_run("tests/lm/atom_comparison_equal.lm"), "True" );
}

#[test]
fn user_defined() {
   assert_eq!( compile_and_run("tests/lm/user_function_unsugared.lm"), "b" );
   assert_eq!( compile_and_run("tests/lm/user_function_sugar1.lm"), "1" );
   assert_eq!( compile_and_run("tests/lm/user_function_sugar2.lm"), "2" );
   assert_eq!( compile_and_run("tests/lm/user_function_sugar3.lm"), "3" );
}
