use failure::ResultExt;
use exitfailure::ExitFailure;
use git2::Repository;

pub fn clone_component_temp(name: &str) -> Result<(), ExitFailure> {
  let url = "https://github.com/HurricaneInteractive/component-template";

  let _repo = Repository::clone(url, name)
    .with_context(|e| format!("Unable to clone respository: {:#?}", e))?;

  Ok(())
}
