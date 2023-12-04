use regex::Regex;

#[test]
fn r1() {
   let re = Regex::new("^[+]").unwrap();
   assert!( re.find_at("+",0).is_some() ); 
   assert!( re.find_at("1+",0).is_none() ); 
}
