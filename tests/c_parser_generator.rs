
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

   let mut grammar = compile("entry-point := λ x:/a/ . slug x");
   assert_eq!(
      grammar.run("entry-point","").to_string(),
      "Parse Error: Expected entry-point at line 1, column 1"
   );
   assert_eq!(
      grammar.run("entry-point","a").to_string(),
      "Parse Result: slug a"
   );
   assert_eq!(
      grammar.run("entry-point","aa").to_string(),
      "Parse Error: Expected entry-point at line 1, column 1"
   );
}
