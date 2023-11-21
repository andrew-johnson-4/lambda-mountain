
use lambda_mountain::parser_generator::compile;

#[test]
fn cfg1() {
   let grammar = compile("entry-point := /a/");
   assert_eq!(
      grammar.run("").to_string(),
      "Error in String at line 1 column 1: expected /a/ in rule entry-point"
   );
   assert_eq!(
      grammar.run("a").to_string(),
      "Success"
   );
   assert_eq!(
      grammar.run("aa").to_string(),
      "Error in String at line 1 column 1: expected EOF in rule entry-point"
   );
}
