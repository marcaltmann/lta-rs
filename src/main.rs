use clap::{Command, crate_name, crate_version, crate_description, crate_authors};

mod config_file;
use crate::config_file::init_batch_config;


fn main() {
    let matches = Command::new(crate_name!())
        .author(crate_authors!("\n"))
        .version(crate_version!())
        .about(crate_description!())
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("init")
                .about("Initializes batch.toml file"),
        )
        .subcommand(
            Command::new("check")
                .about("Does a dry-run"),
        )
        .subcommand(
            Command::new("process")
                .about("Processes the lta for realz"),
        )
        .get_matches();


    match matches.subcommand() {
        Some(("init", _)) => match init_batch_config() {
            Ok(()) => println!("Created {}", config_file::BATCH_CONFIG_FILE),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                eprintln!("{} already exists", config_file::BATCH_CONFIG_FILE);
                std::process::exit(1);
            },
            Err(e) => {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        },
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
