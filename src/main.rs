use structopt::StructOpt;
use exitfailure::ExitFailure;

#[derive(StructOpt)]
struct Cli {
    name: String
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let component_name = &args.name;
    // TODO: Allow to add --no-test --no-stories options
    // TODO: Check if component is already in directory
    // TODO: Add description input option
    // Clone repo into current directory
    react_component_gen::clone_component_temp(component_name)?;
    // Clean .git & .gitignore
    react_component_gen::clean_component_folder(component_name)?;
    // Rename filenames
    react_component_gen::rename_filenames(component_name)?;
    // Edit files to include component name
    react_component_gen::edit_component_name(component_name)?;

    // Notify that changes were successful
    println!("Successfully created: {}", component_name);

    Ok(())
}
