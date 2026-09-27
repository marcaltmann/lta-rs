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
        Some(("init", _sub_matches)) => init_batch_config().unwrap(),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
