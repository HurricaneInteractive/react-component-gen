use exitfailure::ExitFailure;
use std::path::Path;
use std::fs;
use react_component_gen;

fn clean_up_test(name: &str) -> Result<(), ExitFailure> {
    fs::remove_dir_all(Path::new(&name))?;

    Ok(())
}

#[test]
fn test_clone_repo() -> Result<(), ExitFailure> {
    let name = "CloneComponent";
    react_component_gen::clone_component_temp(&name)?;
    assert_eq!(Path::new(&name).exists(), true);

    clean_up_test(&name)?;

    Ok(())
}

#[test]
fn test_file_renaming() -> Result<(), ExitFailure> {
    let name = "RenameComponent";
    react_component_gen::clone_component_temp(&name)?;
    react_component_gen::rename_filenames(&name)?;

    assert_eq!(Path::new(&format!("{}/{}.tsx", name, name)).exists(), true);
    assert_eq!(Path::new(&format!("{}/__test__/{}.test.tsx", name, name)).exists(), true);
    assert_eq!(Path::new(&format!("{}/__spec__/{}.stories.tsx", name, name)).exists(), true);

    clean_up_test(&name)?;

    Ok(())
}

#[test]
fn test_replacing_component_name() -> Result<(), ExitFailure> {
    let name = "ReplaceNameComponent";
    react_component_gen::clone_component_temp(&name)?;
    react_component_gen::rename_filenames(&name)?;
    react_component_gen::edit_component_name(&name)?;

    let readme_contents = fs::read_to_string(Path::new(&format!("{}/README.md", name)))?;
    let readme_test = format!("# {}", name);
    let index_contents = fs::read_to_string(Path::new(&format!("{}/index.ts", name)))?;
    let index_test = format!("export {{ default, {}Props }} from './{}';", name, name);

    assert_eq!(readme_contents.contains(&readme_test), true);
    assert_eq!(index_contents.contains(&index_test), true);

    clean_up_test(&name)?;

    Ok(())
}
