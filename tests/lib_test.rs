use exitfailure::ExitFailure;
use std::path::Path;
use std::fs;
use react_component_gen;

#[test]
fn test_clone_repo() -> Result<(), ExitFailure> {
    let name = "TestComponent";
    react_component_gen::clone_component_temp(&name)?;
    assert_eq!(Path::new(&name).exists(), true);

    fs::remove_dir_all(Path::new(&name))?;

    Ok(())
}
