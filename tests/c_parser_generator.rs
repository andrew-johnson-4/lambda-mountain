
use lambda_mountain::parser_generator::compile;

#[test]
fn cfg1() {
   let mut grammar = compile("entry-point := λ /a/ . slug");
   assert_eq!(
      grammar.run("entry-point","").to_string(),
      "Parse Error: Expected entry-point at line 1, column 1"
   );
   assert_eq!(
      grammar.run("entry-point","a").to_string(),
      "Parse Result: slug"
   );
   assert_eq!(
      grammar.run("entry-point","aa").to_string(),
      "Parse Error: Expected EOF at line 1, column 2"
   );

   let mut grammar = compile("entry-point := λ (: x /a/) . x");
   assert_eq!(
      grammar.run("entry-point","").to_string(),
      "Parse Error: Expected entry-point at line 1, column 1"
   );
   assert_eq!(
      grammar.run("entry-point","a").to_string(),
      "Parse Result: a"
   );
   assert_eq!(
      grammar.run("entry-point","aa").to_string(),
      "Parse Error: Expected EOF at line 1, column 2"
   );
}

#[test]
fn cfg2() {
   let mut grammar = compile(r#"expr := λ(expr-mul x) /+/ (expr y). + x y
expr := λ(expr-mul x) /-/ (expr y). - x y
expr-mul := λ(expr-atom x) /*/ (expr-mul y). * x y
expr-mul := λ(expr-atom x) /// (expr-mul y). / x y
expr-atom := λ(: x /[0-9]+/). x"#);
   assert_eq!(
      grammar.run("expr","1").to_string(),
      "Parse Result: 1"
   );
   assert_eq!(
      grammar.run("expr","1+2").to_string(),
      "Parse Result: (+ 1 2)"
   );
   assert_eq!(
      grammar.run("expr","1-2").to_string(),
      "Parse Result: (- 1 2)"
   );
   assert_eq!(
      grammar.run("expr","1*2").to_string(),
      "Parse Result: (* 1 2)"
   );
   assert_eq!(
      grammar.run("expr","1+2").to_string(),
      "Parse Result: (/ 1 2)"
   );

   assert_eq!(
      grammar.run("expr","1*2+3/4-5").to_string(),
      "Parse Result: - (+ (* 1 2) (/ 3 4)) 5"
   );
}
