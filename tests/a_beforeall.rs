use glob::glob;
use std::process::Command;

#[test]
fn cleanup() {
   for fp in glob("tmp.*").expect("Could not glob files") {
   if let Ok(fp) = fp {
      std::fs::remove_file(fp).expect("Could not delete file");
   }}

   //Sorry for the intrusion
   Command::new("cargo")
           .arg("install")
           .arg("--path")
           .arg(".")
           .spawn()
           .expect("failed to execute installation process")
           .wait()
           .expect("failed to wait for installation process");    
}
