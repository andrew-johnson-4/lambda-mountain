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
   rm("bootstrap.s");
   rm("bootstrap.o");
   let exit = Command::new("as")
                      .stdout(std::process::Stdio::piped())
                      .stderr(std::process::Stdio::piped())
                      .arg("-o")
                      .arg("bootstrap.o")
                      .arg("BOOTSTRAP/cli.s")
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
                      .arg("bootstrap")
                      .arg("bootstrap.o")
                      .spawn()
                      .expect("failed to execute process")
                      .wait_with_output()
                      .expect("failed to wait for process");
   if !exit.status.success() {
      let stderr = String::from_utf8_lossy(&exit.stderr).to_string();
      panic!("ld error code: {}", stderr);
   };
   let exit = Command::new("./bootstrap")
                      .stdout(std::process::Stdio::piped())
                      .stderr(std::process::Stdio::piped())
                      .arg("-o")
                      .arg("bootstrap.s")
                      .arg("BOOTSTRAP/cli.lm")
                      .spawn()
                      .expect("failed to execute process")
                      .wait_with_output()
                      .expect("failed to wait for process");
   if !exit.status.success() {
      let stderr = String::from_utf8_lossy(&exit.stderr).to_string();
      panic!("bootstrap error code: {}", stderr);
   };
}

fn compile_production() {
   compile_bootstrap();
   rm("production");
   rm("production.s");
   rm("production.o");
   let exit = Command::new("./bootstrap")
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

fn run_production(mode:&str, target: &str) -> (String,String) {
   rm("tmp1.s");
   rm("tmp2.s");
   let exit = Command::new("./bootstrap")
                      .stdout(std::process::Stdio::piped())
                      .stderr(std::process::Stdio::piped())
                      .arg(mode)
                      .arg("-o")
                      .arg("tmp1.s")
                      .arg(target)
                      .spawn()
                      .expect("failed to execute process")
                      .wait_with_output()
                      .expect("failed to wait for process");
   if !exit.status.success() {
      let stderr = String::from_utf8_lossy(&exit.stderr).to_string();
      panic!("lm error code while {} {}: {}", mode, target, stderr);
   };
   let expected = String::from_utf8_lossy(&exit.stdout).to_string();
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
   if !exit.status.success() {
      let stderr = String::from_utf8_lossy(&exit.stderr).to_string();
      panic!("./production error code while {} {}: {}", mode, target, stderr);
   };
   let actual = String::from_utf8_lossy(&exit.stdout).to_string();
   (expected, actual)
}

#[test]
fn prodsuite() {
   compile_production();
   let mut failures = Vec::new();
   for entry in glob("tests/lm/*.lm").unwrap() {
      let path = entry.unwrap().display().to_string();
      let (expected,actual) = run_production("--tokenize",&path);
      if expected != actual {
         failures.push(( "--tokenize", path, expected, actual ));
      }
   }
   for entry in glob("BOOTSTRAP/cli.lm").unwrap() {
      let path = entry.unwrap().display().to_string();
      let (expected,actual) = run_production("--tokenize",&path);
      if expected != actual {
         failures.push(( "--tokenize", path, expected, actual ));
      }
   }
   for entry in glob("tests/lm/*.lm").unwrap() {
      let path = entry.unwrap().display().to_string();
      let (expected,actual) = run_production("--parse",&path);
      if expected != actual {
         failures.push(( "--parse", path, expected, actual ));
      }
   }
   for (mode,path,expected,actual) in &failures {
      eprintln!("TEST {} {} Expected: {}, Actual: {}", mode, path, &expected[..std::cmp::min(100,expected.len())], &actual[..std::cmp::min(100,actual.len())]);
   }
   assert_eq!( failures.len(), 0 );
   rm("production");
   rm("production.s");
   rm("production.o");
   rm("bproduction");
   rm("tmp1.s");
   rm("tmp2.s");
}
