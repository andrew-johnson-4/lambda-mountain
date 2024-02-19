use std::process::Command;
use glob::glob;

fn rm(p: &str) {
   if std::path::Path::new(p).is_file() {
      std::fs::remove_file(p).expect(&format!("Could not remove file: {}",p))
   }
   assert!( !std::path::Path::new(p).is_file() );
}

fn compile_production() {
   rm("production");
   rm("production.s");
   rm("production.o");
   let exit = Command::new("lm")
                      .stdout(std::process::Stdio::piped())
                      .stderr(std::process::Stdio::piped())
                      .arg("-o")
                      .arg("production.s")
                      .arg("PRODUCTION/cli.lm")
                      .spawn()
                      .expect("failed to execute process")
                      .wait_with_output()
                      .expect("failed to wait for process");
   if !exit.status.success() {
      let stderr = String::from_utf8_lossy(&exit.stderr).to_string();
      panic!("lm error code: {}", stderr);
   };
   let exit = Command::new("as")
                      .stdout(std::process::Stdio::piped())
                      .stderr(std::process::Stdio::piped())
                      .arg("-o")
                      .arg("production.o")
                      .arg("production.s")
                      .spawn()
                      .expect("failed to execute process")
                      .wait_with_output()
                      .expect("failed to wait for process");
   if !exit.status.success() {
      let stderr = String::from_utf8_lossy(&exit.stderr).to_string();
      panic!("as error code: {}", stderr);
   };
   let exit = Command::new("ld")
                      .stdout(std::process::Stdio::piped())
                      .stderr(std::process::Stdio::piped())
                      .arg("-o")
                      .arg("production")
                      .arg("production.o")
                      .spawn()
                      .expect("failed to execute process")
                      .wait_with_output()
                      .expect("failed to wait for process");
   if !exit.status.success() {
      let stderr = String::from_utf8_lossy(&exit.stderr).to_string();
      panic!("ld error code: {}", stderr);
   };
}
fn run_production(mode:&str, target: &str) -> String {
   let exit = Command::new("./production")
                      .stdout(std::process::Stdio::piped())
                      .stderr(std::process::Stdio::piped())
                      .arg(mode)
                      .arg("-o")
                      .arg("tmp2.s")
                      .arg(target)
                      .spawn()
                      .expect("failed to execute process")
                      .wait_with_output()
                      .expect("failed to wait for process");
   let actual = String::from_utf8_lossy(&exit.stdout).to_string();
   actual
}

#[test]
fn testsuite() {
   compile_production();
   let mut failures = Vec::new();
   for entry in glob("tests/typed/*.lm").unwrap() {
      let path = entry.unwrap().display().to_string();
      let expected = std::fs::read_to_string(path.clone() + ".out")
                    .expect(&format!("Could not load expected output {}.out", path));
      let expected = expected.trim().to_string();
      let actual = run_production("--typecheck", &path);
      let actual = actual.trim().to_string();
      if expected != actual {
         failures.push(( "--typecheck", path, expected, actual ));
      }
   }
   for (mode,path,expected,actual) in &failures {
      eprintln!("TEST {} {}", mode, path);
      eprintln!("Expected: {}", &expected[..std::cmp::min(400,expected.len())] );
      eprintln!("Actual: {}", &actual[..std::cmp::min(400,actual.len())] );
   }
   assert_eq!( failures.len(), 0 );
   rm("production");
   rm("production.s");
   rm("production.o");
}
