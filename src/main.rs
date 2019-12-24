use structopt::StructOpt;
use exitfailure::ExitFailure;

#[derive(StructOpt)]
struct Cli {
    name: String
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let component_name = &args.name;
    let component_file = react_component_gen::load_component_template("template/Component.txt")?;

    println!("{}", component_name);
    println!("{:#?}", component_file);

    Ok(())
}
