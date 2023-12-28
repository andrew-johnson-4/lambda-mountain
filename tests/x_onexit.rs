use glob::glob;

#[test]
fn cleanup() {
   for fp in glob("tmp.*").expect("Could not glob files") {
   if let Ok(fp) = fp {
      std::fs::remove_file(fp).expect("Could not delete file");
   }}
}
