use failure::ResultExt;
use exitfailure::ExitFailure;
use git2::{Repository};
use std::fs;
use regex::Regex;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::{PathBuf};
use std::io::{Write};
use std::path::Path;

/// Clones the template component into the working directory
///
/// # Examples
/// ```ignore
/// use std::io;
///
/// fn main() -> io::Result<()> {
///   react_component_gen::clone_component_temp("Accordion")?;
///   Ok(())
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
/// ```ignore
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
/// ```ignore
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
        if Path::new(&f).exists() {
            let re = Regex::new(r"__COMPONENT__").unwrap();
            let new_filename = re.replace(&f, name).into_owned();

            fs::rename(f, &new_filename)?;
        }
    }

    Ok(())
}

/// Loads a file from a path
///
/// # Examples
/// ```ignore
/// use std::path::{PathBuf};
///
/// let mut path = PathBuf::new();
/// path.push("/path/to/file");
/// let reader = load_file(path);
/// ```
fn load_file(path: &PathBuf) -> Result<BufReader<File>, ExitFailure> {
    let content = File::open(path).with_context(|_| format!("could not read file `{:#?}`", path))?;
    let reader = BufReader::new(content);

    Ok(reader)
}

/// Edit files to replace placeholder component name with proper
/// component name
///
/// # Example
/// ```ignore
/// fn main() {
///   react_component_gen::edit_component_name("Accordion")?;
/// }
/// ```
pub fn edit_component_name(name: &str) -> Result<(), ExitFailure> {
    let files = vec![
        format!("{}/README.md", name),
        format!("{}/{}.tsx", name, name),
        format!("{}/index.ts", name),
        format!("{}/__test__/{}.test.tsx", name, name),
        format!("{}/__spec__/{}.stories.tsx", name, name),
    ];

    // Loop through each file
    for f in files {
        if !Path::new(&f).exists() {
            continue;
        }

        let mut path = PathBuf::new();
        path.push(f);
        let content = load_file(&path)?;
        let mut data = String::new();

        // Loop through lines and push into String
        for l in content.lines() {
            let line = l.unwrap();
            data.push_str(&format!("{}\n", line));
        }

        // replace name and override file
        let new_filedata = data.replace("__COMPONENT__", &name);
        let mut dst = File::create(&path)?;
        dst.write(new_filedata.as_bytes())?;
    }

    Ok(())
}
