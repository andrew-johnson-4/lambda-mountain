
//Compiled Grammars are just special-case left-hand-sides for term bindings

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

