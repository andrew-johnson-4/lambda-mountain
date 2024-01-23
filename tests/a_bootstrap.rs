use std::process::Command;
use glob::glob;

fn compile_bootstrap() {
   let _ = std::fs::remove_file("bootstrap");
   let exit = Command::new("lambda_mountain")
                      .stdout(std::process::Stdio::piped())
                      .stderr(std::process::Stdio::piped())
                      .arg("-o")
                      .arg("bootstrap")
                      .arg("BOOTSTRAP/cli.lm")
                      .spawn()
                      .expect("failed to execute process")
                      .wait_with_output()
                      .expect("failed to wait for process");
   if !exit.status.success() {
      let stderr = String::from_utf8_lossy(&exit.stderr).to_string();
      panic!("lambda_mountain error code: {}", stderr);
   };
}

fn run_bootstrap(target: &str) -> String {
   let _ = std::fs::remove_file("tmp.s");
   let _ = std::fs::remove_file("tmp.o");
   let _ = std::fs::remove_file("a.out");
   let exit = Command::new("./bootstrap")
                      .stdout(std::process::Stdio::piped())
                      .stderr(std::process::Stdio::piped())
                      .arg("-o")
                      .arg("tmp.s")
                      .arg(target)
                      .spawn()
                      .expect("failed to execute process")
                      .wait_with_output()
                      .expect("failed to wait for process");
   if !exit.status.success() {
      let stderr = String::from_utf8_lossy(&exit.stderr).to_string();
      panic!("./bootstrap error code while compiling {}: {}", target, stderr);
   };
   let exit = Command::new("as")
                      .stdout(std::process::Stdio::piped())
                      .stderr(std::process::Stdio::piped())
                      .arg("-o")
                      .arg("tmp.o")
                      .arg("tmp.s")
                      .spawn()
                      .expect("failed to execute process")
                      .wait_with_output()
                      .expect("failed to wait for process");
   if !exit.status.success() {
      let stderr = String::from_utf8_lossy(&exit.stderr).to_string();
      panic!("as error code while compiling {}: {}", target, stderr);
   };
   let exit = Command::new("ld")
                      .stdout(std::process::Stdio::piped())
                      .stderr(std::process::Stdio::piped())
                      .arg("-o")
                      .arg("a.out")
                      .arg("tmp.o")
                      .spawn()
                      .expect("failed to execute process")
                      .wait_with_output()
                      .expect("failed to wait for process");
   if !exit.status.success() {
      let stderr = String::from_utf8_lossy(&exit.stderr).to_string();
      panic!("ld error code while compiling {}: {}", target, stderr);
   };
   let exit = Command::new("./a.out")
                      .stdout(std::process::Stdio::piped())
                      .stderr(std::process::Stdio::piped())
                      .spawn()
                      .expect("failed to execute process")
                      .wait_with_output()
                      .expect("failed to wait for process");
   if !exit.status.success() {
      let stderr = String::from_utf8_lossy(&exit.stderr).to_string();
      panic!("./a.out error code while running {}: {}", target, stderr);
   };
   String::from_utf8_lossy(&exit.stdout).to_string()
}

//#[test]
fn suite() {
   compile_bootstrap();
   for entry in glob("tests/lm/*.lm").unwrap() {
      let path = entry.unwrap().display().to_string();
      let stdout = path.clone() + ".out";
      assert!(std::path::Path::new(&stdout).exists(),"Expected stdout not found: {}",stdout);
      let stdout = std::fs::read_to_string(stdout).unwrap();
      let stdout = stdout.trim();
      let actual = run_bootstrap(&path);
      assert_eq!(stdout, actual, "Expected: {}, Actual: {}", stdout, actual);
   }
}
