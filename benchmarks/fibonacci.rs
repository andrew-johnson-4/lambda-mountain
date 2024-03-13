
fn fibonacci(n: u64) -> u64 {
    if n<=2 { 1 } else { fibonacci(n-1) + fibonacci(n-2) }
}

fn main() {
   for argument in std::env::args().skip(1) {
      fibonacci( argument.parse::<u64>().unwrap_or(0) );
   }
}
