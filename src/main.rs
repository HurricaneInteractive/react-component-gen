use structopt::StructOpt;
use exitfailure::ExitFailure;
use std::path::Path;

#[derive(StructOpt)]
struct Cli {
    name: String,

    #[structopt(short = "d", long = "description", default_value = "")]
    description: String,

    #[structopt(short = "t", long = "test")]
    test: bool,

    #[structopt(short = "s", long = "stories")]
    stories: bool,
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let component_name = &args.name;
    let _desc = &args.description;
    let remove_tests = &args.test;
    let remove_stories = &args.stories;

    // Check if component is already in directory
    if Path::new(&component_name).exists() {
        println!("Component '{}' already exists", component_name);
        return Ok(());
    }

    // Clone repo into current directory
    react_component_gen::clone_component_temp(component_name)?;

    // Clean .git & .gitignore
    react_component_gen::clean_component_folder(component_name)?;

    // Remove __test__ if -t is defined
    if *remove_tests {
        react_component_gen::remove_tests(component_name)?;
    }

    // Remove __spec__ if -s is defined
    if *remove_stories {
        react_component_gen::remove_specs(component_name)?;
    }

    // Rename filenames
    react_component_gen::rename_filenames(component_name)?;

    // TODO: Add description input option
    // react_component_gen::add_desc_to_readme(component_name)?;

    // Edit files to include component name
    react_component_gen::edit_component_name(component_name)?;

    // Notify that changes were successful
    println!("Successfully created: {}", component_name);

    Ok(())
}
