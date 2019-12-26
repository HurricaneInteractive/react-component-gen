use failure::ResultExt;
use exitfailure::ExitFailure;
use git2::{Repository};
use std::fs;
use regex::Regex;

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

/// Renames all __COMPONENT__.** files using the component name
///
/// # Example
/// ```no_run
/// fn main() {
///   react_component_gen::rename_filenames("Accordion")?;
/// }
/// ```
pub fn rename_filenames(name: &str) -> Result<(), ExitFailure> {
  let placeholder = "__COMPONENT__";
  let files = vec![
    format!("{}/{}.tsx", name, placeholder),
    format!("{}/__test__/{}.test.tsx", name, placeholder),
    format!("{}/__spec__/{}.stories.tsx", name, placeholder)
  ];

  for f in files {
    let re = Regex::new(r"__COMPONENT__").unwrap();
    let new_filename = re.replace(&f, name).into_owned();

    fs::rename(f, &new_filename)?;
  }

  Ok(())
}
