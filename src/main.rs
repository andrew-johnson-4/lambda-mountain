
fn main() {
   let mut hard = true;
   for arg in std::env::args().skip(1) {
      if arg=="hard" { hard = true; }
      else if arg=="soft" { hard = false; }
      else if hard { eval_hard(&arg); }
      else         { eval_soft(&arg); }
   }
}

fn eval_soft(filename: &str) {
   println!("eval soft {filename}");
}

fn eval_hard(filename: &str) {
   println!("eval hard {filename}");
}
