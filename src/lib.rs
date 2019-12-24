use std::io::{BufReader};
use std::fs::File;
use failure::ResultExt;
use exitfailure::ExitFailure;

pub fn load_component_template(path: &str) -> Result<BufReader<File>, ExitFailure> {
  let file = File::open(&path).with_context(|_| format!("Could not open file `{}`", path))?;
  let reader = BufReader::new(file);

  Ok(reader)
}
