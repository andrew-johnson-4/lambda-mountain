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
                      .arg(target)
                      .arg("-o")
                      .arg("bootstrapped.out")
                      .spawn()
                      .expect("failed to execute process")
                      .wait_with_output()
                      .expect("failed to wait for process");
   if !exit.status.success() {
      let stderr = String::from_utf8_lossy(&exit.stderr).to_string();
      return format!("./a.out error code: {}", stderr);
   };
   String::from_utf8_lossy(&exit.stdout).to_string()
   /* TODO
   let exit = Command::new("./bootstrapped.out")
                      .stdout(std::process::Stdio::piped())
                      .stderr(std::process::Stdio::piped())
                      .spawn()
                      .expect("failed to execute process")
                      .wait_with_output()
                      .expect("failed to wait for process");
   if !exit.status.success() {
      let stderr = String::from_utf8_lossy(&exit.stderr).to_string();
      return format!("./bootstrapped.out error code: {}", stderr);
   };
   String::from_utf8_lossy(&exit.stdout).to_string()
   */
}

#[test]
fn cli_helloworld() {
   assert_eq!( compile_and_run("tests/lm/hello_world.lm"), "hello_world" );
}
