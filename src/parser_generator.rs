
//Type-0 Grammar Rules
struct Rule {
   name: String,
   string: Vec<Symbol>,
}

//Symbols in Production Rules
enum Symbol {
   Bind(String,String),
   Regex(String),
   Scan(String,String,String),
   Descend(String),
}
