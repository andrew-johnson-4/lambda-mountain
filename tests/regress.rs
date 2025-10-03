use std::process::Command;
use glob::glob;

fn rm(p: &str) {
   if std::path::Path::new(p).is_file() {
      std::fs::remove_file(p).expect(&format!("Could not remove file: {}",p))
   }
   assert!( !std::path::Path::new(p).is_file() );
}

fn compile_bootstrap() {
   rm("bootstrap.exe");
   let exit = Command::new("cc")
                      .stdout(std::process::Stdio::piped())
                      .stderr(std::process::Stdio::piped())
                      .arg("-w")
                      .arg("-O2")
                      .arg("-march=native")
                      .arg("-mtune=native")
                      .arg("-o")
                      .arg("bootstrap.exe")
                      .arg("BOOTSTRAP/cli.c")
                      .spawn()
                      .expect("failed to execute process")
                      .wait_with_output()
                      .expect("failed to wait for process");
   if !exit.status.success() {
      let stderr = String::from_utf8_lossy(&exit.stderr).to_string();
      panic!("cc error code: {}", stderr);
   };
}

fn run_bootstrap(target: &str, leave_tmp: bool, is_v3: bool) -> String {
   if !leave_tmp { rm("tmp.c"); };
   rm("a.out");
   
   let exit = if is_v3 {
      Command::new("./bootstrap.exe")
              .stdout(std::process::Stdio::piped())
              .stderr(std::process::Stdio::piped())
              .arg("--v3")
              .arg("-o")
              .arg("tmp.c")
              .arg(target)
              .spawn()
              .expect("failed to execute process")
              .wait_with_output()
              .expect("failed to wait for process")
   } else {
      Command::new("./bootstrap.exe")
              .stdout(std::process::Stdio::piped())
              .stderr(std::process::Stdio::piped())
              .arg("--v2")
              .arg("-o")
              .arg("tmp.c")
              .arg(target)
              .spawn()
              .expect("failed to execute process")
              .wait_with_output()
              .expect("failed to wait for process")
   };

   let mut output = "".to_string();
   if !exit.status.success() {
      output = "Compilation Error: ".to_owned() + &String::from_utf8_lossy(&exit.stdout).to_string()
                                    + &String::from_utf8_lossy(&exit.stderr).to_string();
   };
   if output=="" {
      let exit = Command::new("cc")
                      .stdout(std::process::Stdio::piped())
                      .stderr(std::process::Stdio::piped())
                      .arg("-o")
                      .arg("a.out")
                      .arg("tmp.c")
                      .spawn()
                      .expect("failed to execute process")
                      .wait_with_output()
                      .expect("failed to wait for process");
      if !exit.status.success() {
         let stderr = String::from_utf8_lossy(&exit.stderr).to_string();
         return format!("cc error code: {} on target {}", stderr, target);
      };
      let exit = Command::new("timeout")
                      .stdout(std::process::Stdio::piped())
                      .stderr(std::process::Stdio::piped())
                      .arg("30")
                      .arg("./a.out")
                      .spawn()
                      .expect("failed to execute process")
                      .wait_with_output()
                      .expect("failed to wait for process");
      output = if exit.status.success() {
         String::from_utf8_lossy(&exit.stdout).to_string()
      } else {
         "Runtime Error: ".to_owned() + &String::from_utf8_lossy(&exit.stdout).to_string()
                              + &String::from_utf8_lossy(&exit.stderr).to_string()
      };
   }
   if !leave_tmp { rm("tmp.c"); };
   rm("a.out");
   output
}

#[test]
fn regression_tests() {
   compile_bootstrap();
   let mut failures = Vec::new();
   for entry in glob("tests/regress/*.lm").unwrap() {
      let path = entry.unwrap().display().to_string();
      if !std::path::Path::new(&(path.clone() + ".skip")).exists() {
         let expected = std::fs::read_to_string(path.clone() + ".out")
                       .expect(&format!("Could not load expected output {}.out", path));
         let expected = expected.trim().to_string();
         let actual = run_bootstrap(&path, false, false);
         let actual = actual.trim().to_string();
         if expected != actual {
            failures.push(( "--compile".to_string(), path, expected, actual ));
         }
      }
   }
   for entry in glob("tests/regress/*.lsts").unwrap() {
      let path = entry.unwrap().display().to_string();
      if !std::path::Path::new(&(path.clone() + ".skip")).exists() {
         let expected = std::fs::read_to_string(path.clone() + ".out")
                       .expect(&format!("Could not load expected output {}.out", path));
         let expected = expected.trim().to_string();
         let actual = run_bootstrap(&path, false, false);
         let actual = actual.trim().to_string();
         if expected != actual {
            failures.push(( "--compile".to_string(), path, expected, actual ));
         }
      }
   }
   for entry in glob("tests/lib/*.lsts").unwrap() {
      let path = entry.unwrap().display().to_string();
      if !std::path::Path::new(&(path.clone() + ".skip")).exists() {
         let expected = std::fs::read_to_string(path.clone() + ".out")
                       .expect(&format!("Could not load expected output {}.out", path));
         let expected = expected.trim().to_string();
         let actual = run_bootstrap(&path, false, false);
         let actual = actual.trim().to_string();
         if expected != actual {
            failures.push(( "--compile".to_string(), path, expected, actual ));
         }
      }
   }
   for entry in glob("tests/unit/*.lsts").unwrap() {
      let path = entry.unwrap().display().to_string();
      if !std::path::Path::new(&(path.clone() + ".skip")).exists() {
         let expected = std::fs::read_to_string(path.clone() + ".out")
                       .expect(&format!("Could not load expected output {}.out", path));
         let expected = expected.trim().to_string();
         let actual = run_bootstrap(&path, false, false);
         let actual = actual.trim().to_string();
         if expected != actual {
            failures.push(( "--compile".to_string(), path, expected, actual ));
         }
      }
   }
   for entry in glob("tests/promises/*/*.lsts").unwrap() {
      let path = entry.unwrap().display().to_string();
      if !std::path::Path::new(&(path.clone() + ".skip")).exists() {
         let expected = std::fs::read_to_string(path.clone() + ".out").unwrap_or("".to_string());
         let expected = expected.trim().to_string();
         let isv3 = !path.contains("lm-");
         let actual = run_bootstrap(&path, false, isv3);
         let actual = actual.trim().to_string();
         if expected != actual {
            failures.push(( format!("--compile {}", isv3), path, expected, actual ));
         }
      }
   }
   for (mode,path,expected,actual) in &failures {
      eprintln!("TEST {} {}", mode, path);
      eprintln!("Expected: '{}'", &expected[..std::cmp::min(4000,expected.len())] );
      eprintln!("Actual: '{}'", &actual[..std::cmp::min(4000,actual.len())] );
   }
   assert_eq!( failures.len(), 0 );
}
