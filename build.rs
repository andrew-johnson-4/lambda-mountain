fn main() {
   std::process::Command::new("cp")
      .args(&["src/main.rs", "src/bin/lm/main.rs"])
      .status()
      .expect("Copy main.rs failed to execute.");
      ()
}
