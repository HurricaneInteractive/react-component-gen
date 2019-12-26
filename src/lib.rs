use failure::ResultExt;
use exitfailure::ExitFailure;
use git2::{Repository};
use std::fs;

/// Clones the template component into the working directory
///
/// # Examples
/// ```no_run
/// fn main() {
///   react_component_gen::clone_component_temp("Accordion")?;
/// }
/// ```
pub fn clone_component_temp(name: &str) -> Result<(), ExitFailure> {
  let url = "https://github.com/HurricaneInteractive/component-template";

  let _repo = Repository::clone(url, name)
    .with_context(|e| format!("Unable to clone respository: {:#?}", e))?;

  Ok(())
}

/// Removes the .git folder and .gitignore file from the generated
/// component structure
///
/// # Examples
/// ```no_run
/// fn main() {
///   react_component_gen::clean_component_folder("Accordion")?;
/// }
/// ```
pub fn clean_component_folder(name: &str) -> Result<(), ExitFailure> {
  fs::remove_dir_all(format!("{}/.git", name))?;
  fs::remove_file(format!("{}/.gitignore", name))?;

  Ok(())
}
