use std::process::Command;
use glob::glob;

fn rm(p: &str) {
   if std::path::Path::new(p).is_file() {
      std::fs::remove_file(p).expect(&format!("Could not remove file: {}",p))
   }
   assert!( !std::path::Path::new(p).is_file() );
}

fn compile_bootstrap() {
   rm("bootstrap");
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
   rm("tmp.s");
   rm("tmp.o");
   rm("a.out");
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
      return format!("./bootstrap error code while compiling {}: {}", target, stderr);
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
      return format!("as error code while compiling {}: {}", target, stderr);
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
      return format!("ld error code while compiling {}: {}", target, stderr);
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
      return format!("./a.out error code while running {}: {}", target, stderr);
   };
   String::from_utf8_lossy(&exit.stdout).to_string()
}

#[test]
fn suite() {
   compile_bootstrap();
   let mut failures = Vec::new();
   for entry in glob("tests/lm/*.lm").unwrap() {
      let path = entry.unwrap().display().to_string();
      let stdout = path.clone() + ".out";
      assert!(std::path::Path::new(&stdout).exists(),"Expected stdout not found: {}",stdout);
      let stdout = std::fs::read_to_string(stdout).unwrap();
      let stdout = stdout.trim().to_string();
      let actual = run_bootstrap(&path);
      if stdout != actual {
         failures.push(( path, stdout, actual ));
      }
   }
   for (path,stdout,actual) in &failures {
      eprintln!("TEST {} Expected: {}, Actual: {}", path, &stdout[..100], &actual[..100]);
   }
   assert_eq!( failures.len(), 0 );
}
