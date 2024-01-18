use std::process::Command;

fn compile_and_run(target: &str) -> String {
   let exit = Command::new("lambda_mountain")
                      .stdout(std::process::Stdio::piped())
                      .stderr(std::process::Stdio::piped())
                      .arg("BOOTSTRAP/cli.lm")
                      .spawn()
                      .expect("failed to execute process")
                      .wait_with_output()
                      .expect("failed to wait for process");
   if !exit.status.success() {
      let stderr = String::from_utf8_lossy(&exit.stderr).to_string();
      return format!("lambda_mountain error code: {}", stderr);
   };
   let exit = Command::new("./a.out")
                      .stdout(std::process::Stdio::piped())
                      .stderr(std::process::Stdio::piped())
                      .arg("--parse")
                      .arg(target)
                      .spawn()
                      .expect("failed to execute process")
                      .wait_with_output()
                      .expect("failed to wait for process");
   if !exit.status.success() {
      let stderr = String::from_utf8_lossy(&exit.stderr).to_string();
      return format!("./a.out error code: {}", stderr);
   };
   String::from_utf8_lossy(&exit.stdout).to_string()
}

#[test]
fn cli_parser() {
   assert_eq!( compile_and_run("tests/lm/parse_variable1.lm"), "(Variable abc)" );
   assert_eq!( compile_and_run("tests/lm/parse_literal1.lm"), "(Literal 123)" );
   assert_eq!( compile_and_run("tests/lm/parse_literal2.lm"), "(Literal abc)" );
}
