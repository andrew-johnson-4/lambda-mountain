use std::process::Command;

fn compile_and_run(fp: &str) -> String {
   let exit = Command::new("lambda_mountain")
                      .stdout(std::process::Stdio::piped())
                      .stderr(std::process::Stdio::piped())
                      .arg(fp)
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
fn cli_cli() {
   assert_eq!( compile_and_run("tests/lm/hello_world.lm"), "hello_world" );
}


#[test]
fn cli_yield() {
   assert_eq!( compile_and_run("tests/lm/nil.lm"), "()" );
   assert_eq!( compile_and_run("tests/lm/123.lm"), "123" );
   assert_eq!( compile_and_run("tests/lm/cons.lm"), "((123 ()) 456)" );
}

#[test]
fn cli_headtail() {
   assert_eq!( compile_and_run("tests/lm/head.lm"), "123" );
   assert_eq!( compile_and_run("tests/lm/head_is_nil.lm"), "()" );
   assert_eq!( compile_and_run("tests/lm/tail.lm"), "123" );
   assert_eq!( compile_and_run("tests/lm/tail_is_nil.lm"), "()" );
}

#[test]
fn cli_comparison() {
   assert_eq!( compile_and_run("tests/lm/atom_comparison_not.lm"), "True" );
   assert_eq!( compile_and_run("tests/lm/atom_comparison_notnot.lm"), "()" );
   assert_eq!( compile_and_run("tests/lm/atom_comparison_inequal.lm"), "()" );
   assert_eq!( compile_and_run("tests/lm/atom_comparison_equal.lm"), "True" );
   assert_eq!( compile_and_run("tests/lm/atom_comparison_equal2.lm"), "(True λ)" );
}

#[test]
fn user_defined() {
   assert_eq!( compile_and_run("tests/lm/user_function_unsugared.lm"), "b" );
   assert_eq!( compile_and_run("tests/lm/user_function_sugar1.lm"), "1" );
   assert_eq!( compile_and_run("tests/lm/user_function_sugar2.lm"), "2" );
   assert_eq!( compile_and_run("tests/lm/user_function_sugar3.lm"), "3" );
}

#[test]
fn helpers() {
   assert_eq!( compile_and_run("tests/lm/sequence.lm"), "123" );
   assert_eq!( compile_and_run("tests/lm/locals.lm"), "()" );
   assert_eq!( compile_and_run("tests/lm/locals2.lm"), "123" );
   assert_eq!( compile_and_run("tests/lm/locals3.lm"), "()" );
   assert_eq!( compile_and_run("tests/lm/locals4.lm"), "()" );
   assert_eq!( compile_and_run("tests/lm/assign.lm"), "((123 abc) xyz)" );
   assert_eq!( compile_and_run("tests/lm/assign2.lm"), "(((() 123) abc) xyz)" );
}

#[test]
fn control_flow() {
   assert_eq!( compile_and_run("tests/lm/if_truthy.lm"), "True" );
   assert_eq!( compile_and_run("tests/lm/if_falsy.lm"), "False" );
   assert_eq!( compile_and_run("tests/lm/match0.lm"), "()" );
   assert_eq!( compile_and_run("tests/lm/match1.lm"), "1" );
   assert_eq!( compile_and_run("tests/lm/match2.lm"), "2" );
   assert_eq!( compile_and_run("tests/lm/match3.lm"), "456" );
   assert_eq!( compile_and_run("tests/lm/match4.lm"), "(1 3)" );
   assert_eq!( compile_and_run("tests/lm/match5.lm"), "1" );
   assert_eq!( compile_and_run("tests/lm/match6.lm"), "(1 2)" );
   assert_eq!( compile_and_run("tests/lm/match7.lm"), "1" );
   assert_eq!( compile_and_run("tests/lm/match8.lm"), "(1 3)" );
   assert_eq!( compile_and_run("tests/lm/match9.lm"), "((1 2) 3)" );
}

#[test]
fn dsa() {
   assert_eq!( compile_and_run("tests/lm/concat.lm"), "(((() 1) 2) 3)" );
   assert_eq!( compile_and_run("tests/lm/kv_merge.lm"), "(((((() (1 2)) (3 4)) (5 6)) (7 8)) (9 0))" );
   assert_eq!( compile_and_run("tests/lm/kv_lookup.lm"), "456" );
   assert_eq!( compile_and_run("tests/lm/kv_lookup2.lm"), "000" );
}

#[test]
fn eval_soft() {
   assert_eq!( compile_and_run("tests/lm/destructure2.lm"), "()" );
   assert_eq!( compile_and_run("tests/lm/destructure.lm"), "((CTX (b 1)) (c 3))" );
   assert_eq!( compile_and_run("tests/lm/eval_cons.lm"), "((123 ()) abc)" );
   assert_eq!( compile_and_run("tests/lm/eval_substitution.lm"), "123" );
   assert_eq!( compile_and_run("tests/lm/eval_lambda.lm"), "(123 123)" );
   assert_eq!( compile_and_run("tests/lm/eval_lambda2.lm"), "(123 456)" );
}

#[test]
fn rope() {
   assert_eq!( compile_and_run("tests/lm/clone_rope2.lm"), "(() (A (B C)))" );
   assert_eq!( compile_and_run("tests/lm/clone_rope3.lm"), "(((() A) B) C)" );
   assert_eq!( compile_and_run("tests/lm/clone_rope.lm"), "helloworld" );
   assert_eq!( compile_and_run("tests/lm/foreach_atom.lm"), "helloworld" );
   assert_eq!( compile_and_run("tests/lm/foreach_char.lm"), "h_e_l_l_o_w_o_r_l_d_" );
   assert_eq!( compile_and_run("tests/lm/foreach_atom_e.lm"), "hello_world_" );
   assert_eq!( compile_and_run("tests/lm/foreach_char_e.lm"), "h_e_l_l_o_w_o_r_l_d_" );
}

#[test]
fn system_calls() {
   assert_eq!( compile_and_run("tests/lm/argv.lm"), "(./a.out ())" );
   assert_eq!( compile_and_run("tests/lm/load_file.lm"), "helloworld\n" );
   let bigger_contents = std::fs::read_to_string("tests/lm/bigger_file.txt")
                        .expect("Could not read tests/lm/bigger_file.txt");
   assert_eq!( compile_and_run("tests/lm/load_bigger_file.lm"), bigger_contents );
}
