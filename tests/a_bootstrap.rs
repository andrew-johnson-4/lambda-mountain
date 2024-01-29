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
   rm("bbootstrap");
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
   let exit = Command::new("as")
                      .stdout(std::process::Stdio::piped())
                      .stderr(std::process::Stdio::piped())
                      .arg("-o")
                      .arg("bootstrap.o")
                      .arg("bootstrap.s")
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
                      .arg("bbootstrap")
                      .arg("bootstrap.o")
                      .spawn()
                      .expect("failed to execute process")
                      .wait_with_output()
                      .expect("failed to wait for process");
   if !exit.status.success() {
      let stderr = String::from_utf8_lossy(&exit.stderr).to_string();
      panic!("ld error code: {}", stderr);
   };
}

fn run_bootstrap(mode:&str, target: &str) -> (String,String) {
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
      panic!("./bootstrap error code while {} {}: {}", mode, target, stderr);
   };
   let expected = String::from_utf8_lossy(&exit.stdout).to_string();
   let exit = Command::new("./bbootstrap")
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
      panic!("./bbootstrap error code while {} {}: {}", mode, target, stderr);
   };
   let actual = String::from_utf8_lossy(&exit.stdout).to_string();
   (expected, actual)
}

#[test]
fn bootsuite() {
   compile_bootstrap();
   let mut failures = Vec::new();
   for entry in glob("tests/lm/*.lm").unwrap() {
      let path = entry.unwrap().display().to_string();
      let (expected,actual) = run_bootstrap("--tokenize",&path);
      if expected != actual {
         failures.push(( "--tokenize", path, expected, actual ));
      }
   }
   for entry in glob("tests/lm/*.lm").unwrap() {
      let path = entry.unwrap().display().to_string();
      let (expected,actual) = run_bootstrap("--parse",&path);
      if expected != actual {
         failures.push(( "--parse", path, expected, actual ));
      }
   }
   for entry in glob("tests/lm/*.lm").unwrap() {
      let path = entry.unwrap().display().to_string();
      let (expected,actual) = run_bootstrap("--compile",&path);
      if expected != actual {
         failures.push(( "--compile", path, expected, actual ));
      }
   }
   for entry in glob("BOOTSTRAP/cli.lm").unwrap() {
      let path = entry.unwrap().display().to_string();
      let (expected,actual) = run_bootstrap("--tokenize",&path);
      if expected != actual {
         failures.push(( "--tokenize", path, expected, actual ));
      }
   }
   for entry in glob("BOOTSTRAP/cli.lm").unwrap() {
      let path = entry.unwrap().display().to_string();
      let (expected,actual) = run_bootstrap("--parse",&path);
      if expected != actual {
         failures.push(( "--parse", path, expected, actual ));
      }
   }
   for entry in glob("BOOTSTRAP/cli.lm").unwrap() {
      let path = entry.unwrap().display().to_string();
      let (expected,actual) = run_bootstrap("--compile",&path);
      if expected != actual {
         failures.push(( "--compile", path, expected, actual ));
      }
   }
   for (mode,path,expected,actual) in &failures {
      eprintln!("TEST {} {} Expected: {}, Actual: {}", mode, path, &expected[..std::cmp::min(100,expected.len())], &actual[..std::cmp::min(100,actual.len())]);
   }
   assert_eq!( failures.len(), 0 );
   rm("bootstrap");
   rm("bootstrap.s");
   rm("bootstrap.o");
   rm("bbootstrap");
   rm("tmp1.s");
   rm("tmp2.s");
}
