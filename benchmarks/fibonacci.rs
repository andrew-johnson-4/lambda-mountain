
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        2 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
   for argument in std::env::args().skip(1) {
      fibonacci( argument.parse::<u64>().unwrap_or(0) );
   }
}
