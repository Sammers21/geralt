use geralt::config;

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
            geralt::init(".");
        }
        Some(("build", _)) => {
            geralt::build(config::read_toml("."));
        }
        Some(("run", _)) => {
            geralt::run(config::read_toml("."));
        }
        _ => println!("No subcommand provided"),
    };
}
