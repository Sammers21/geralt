
pub fn main() {
    match clap::Command::new("geralt")
        .bin_name("geralt")
        .subcommand_required(true)
        .subcommand(clap::command!("init"))
        .subcommand(clap::command!("build"))
        .subcommand(clap::command!("run"))
        .get_matches()
        .subcommand()
    {
        Some(("init", _)) => {
            println!("Created binary (application) package");
            geralt::init(".");
        }
        Some(("build", _)) => println!("Building the fat jar..."),
        Some(("run", _)) => println!("Running the application..."),
        _ => println!("No subcommand provided"),
    };
}