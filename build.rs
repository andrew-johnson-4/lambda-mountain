fn main() {
   println!("cargo:rerun-if-changed=BOOTSTRAP/cli.s");
   std::process::Command::new("as")
      .args(&["-o","lm", "BOOTSTRAP/cli.s"])
      .status()
      .expect("as BOOTSTRAP/cli.s failed to execute.");
      ()
}
