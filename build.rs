fn main() {
   println!("cargo:rerun-if-changed=BOOTSTRAP/cli.s");
   std::process::Command::new("as")
      .args(&["-o","lm_raw.o", "BOOTSTRAP/cli.s"])
      .status()
      .expect("as BOOTSTRAP/cli.s failed to execute.");
   std::process::Command::new("ld")
      .args(&["-o","lm_raw", "lm_raw.o"])
      .status()
      .expect("ld BOOTSTRAP/cli.s failed to execute.");
   std::process::Command::new("rm")
      .args(&["lm_raw.o"])
      .status()
      .expect("rm lm_raw.o failed to execute.");
   std::process::Command::new("mv")
      .args(&["lm_raw","$HOME/.cargo/bin/lm"])
      .status()
      .expect("rm lm_raw.o failed to execute.");
   ()
}
