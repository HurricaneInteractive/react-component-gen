use structopt::StructOpt;
use exitfailure::ExitFailure;

#[derive(StructOpt)]
struct Cli {
    name: String
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let component_name = &args.name;
    // Clone repo into current directory
    react_component_gen::clone_component_temp(component_name)?;
    // Clean .git & .gitignore
    // Rename filenames
    // Edit files to include component name
    // Notify that changes were successful

    println!("{}", component_name);

    Ok(())
}
