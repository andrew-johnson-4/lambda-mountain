fn main() {
   std::process::Command::new("as")
      .args(&["-o","lm_raw.o", "BOOTSTRAP/cli.s"])
      .status()
      .expect("as BOOTSTRAP/cli.s failed to execute.");
   std::process::Command::new("ld")
      .args(&["-o","lm", "lm_raw.o"])
      .status()
      .expect("ld BOOTSTRAP/cli.s failed to execute.");
   std::process::Command::new("mv")
      .args(&["lm","${HOME}/.cargo/bin/"])
      .status()
      .expect("mv lm ${HOME}/.cargo/bin/ failed to execute.");
   std::process::Command::new("rm")
      .args(&["lm_raw.o"])
      .status()
      .expect("rm lm_raw.o failed to execute.");
   ()
}
