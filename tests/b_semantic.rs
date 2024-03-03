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

fn run_production(mode:&str, target: &str) -> String {
   let exit = Command::new("timeout")
                      .stdout(std::process::Stdio::piped())
                      .stderr(std::process::Stdio::piped())
                      .arg("5")
                      .arg("./production")
                      .arg(mode)
                      .arg(target)
                      .spawn()
                      .expect("failed to execute process")
                      .wait_with_output()
                      .expect("failed to wait for process");
   let actual = String::from_utf8_lossy(&exit.stdout).to_string();
   actual
}

fn run_compile_production(mode:&str, target: &str) -> String {
   rm("tmp.s");
   rm("tmp.o");
   rm("a.out");
   let exit = Command::new("timeout")
                      .stdout(std::process::Stdio::piped())
                      .stderr(std::process::Stdio::piped())
                      .arg("5")
                      .arg("./production")
                      .arg(mode)
                      .arg("-o")
                      .arg("tmp.s")
                      .arg("STDLIB/default-rules.lm")
                      .arg(target)
                      .spawn()
                      .expect("failed to execute process")
                      .wait_with_output()
                      .expect("failed to wait for process");
   if !exit.status.success() {
      let stderr = String::from_utf8_lossy(&exit.stderr).to_string();
      return format!("timeout 5 ./production error code: {} on target {}", stderr, target);
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
      return format!("as error code: {} on target {}", stderr, target);
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
      return format!("ld error code: {} on target {}", stderr, target);
   };
   let exit = Command::new("timeout")
                      .stdout(std::process::Stdio::piped())
                      .stderr(std::process::Stdio::piped())
                      .arg("5")
                      .arg("./a.out")
                      .spawn()
                      .expect("failed to execute process")
                      .wait_with_output()
                      .expect("failed to wait for process");
   if !exit.status.success() {
      let stderr = String::from_utf8_lossy(&exit.stderr).to_string();
      return format!("timeout 5 ./a.out error code: {} on target {}", stderr, target);
   };
   let actual = String::from_utf8_lossy(&exit.stdout).to_string();
   rm("tmp.s");
   rm("tmp.o");
   rm("a.out");
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
   for entry in glob("tests/lm/*.lm").unwrap() {
      let path = entry.unwrap().display().to_string();
      let expected = std::fs::read_to_string(path.clone() + ".out")
                    .expect(&format!("Could not load expected output {}.out", path));
      let expected = expected.trim().to_string();
      let actual = run_compile_production("--compile", &path);
      let actual = actual.trim().to_string();
      if expected != actual {
         failures.push(( "--compile", path, expected, actual ));
      }
   }
   for entry in glob("tests/strict/*.lm").unwrap() {
      let path = entry.unwrap().display().to_string();
      let expected = std::fs::read_to_string(path.clone() + ".out")
                    .expect(&format!("Could not load expected output {}.out", path));
      let expected = expected.trim().to_string();
      let actual = run_compile_production("--strict", &path);
      let actual = actual.trim().to_string();
      if expected != actual {
         failures.push(( "--strict", path, expected, actual ));
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
