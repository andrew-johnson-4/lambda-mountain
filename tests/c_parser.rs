use std::process::Command;

fn compile_and_run(mode: &str, target: &str) -> String {
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
                      .arg(mode)
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
fn cli_tokenizer() {
   assert_eq!( compile_and_run("--tokenize","tests/lm/parse_variable1.lm"), "(() abc)" );
   assert_eq!( compile_and_run("--tokenize","tests/lm/parse_literal1.lm"), "(() 123)" );
   assert_eq!( compile_and_run("--tokenize","tests/lm/parse_literal2.lm"), "((() ') abc)" );
   assert_eq!( compile_and_run("--tokenize","tests/lm/parse_literal3.lm"), "((((((((((((((((() abc) 123) :) :) =) () )) () xy) ;) z) .) λ) λ) ') d)" );
}

#[test]
fn cli_parse_expression() {
   assert_eq!( compile_and_run("--parse-expression","tests/lm/parse_variable1.lm"), "(Variable abc)" );
   assert_eq!( compile_and_run("--parse-expression","tests/lm/parse_literal1.lm"), "(Literal 123)" );
   assert_eq!( compile_and_run("--parse-expression","tests/lm/parse_literal2.lm"), "(Literal abc)" );
   assert_eq!( compile_and_run("--parse-expression","tests/lm/parse_application.lm"), "(App ((App ((Variable x) (Variable y))) (Variable z)))" );
   assert_eq!( compile_and_run("--parse-expression","tests/lm/parse_nil.lm"), "Nil" );
   assert_eq!( compile_and_run("--parse-expression","tests/lm/parse_grouped.lm"), "(App ((App ((Variable w) (Variable x))) (App ((Variable y) (Variable z)))))" );
   assert_eq!( compile_and_run("--parse-expression","tests/lm/parse_lambda.lm"), "(Lambda ((Variable x) (Variable y)))" );
   assert_eq!( compile_and_run("--parse-expression","tests/lm/parse_lambda2.lm"), "(Lambda (((Variable x) (Variable y)) (App ((App ((Variable w) (Variable x))) (App ((Variable y) (Variable z)))))))" );
}

#[test]
fn cli_parse_program() {
   assert_eq!( compile_and_run("--parse","tests/lm/parse_program.lm"), "((() (x (Variable y))) (a (Variable b)))" );
}
