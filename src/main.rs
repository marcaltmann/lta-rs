use std::io;
use std::io::Write;
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
        Some(("init", _)) => init(),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}

fn init() {
    let mut batch_id = String::new();
    let mut archive_base_url = String::new();

    let stdin = io::stdin();

    println!("This utility will walk you through creating a batch.toml file.");
    println!("It only covers the most common items, and tries to guess sensible defaults.\n");
    println!("See `lta help init` for definitive documentation on these fields and exactly what they do.\n");
    println!("Use `lta archive` afterwards to start the archiving process.\n");

    println!("Press ^C at any time to quit.");

    print!("batch id: ");
    io::stdout().flush().unwrap();
    stdin.read_line(&mut batch_id).unwrap();
    let trimmed_batch_id = batch_id.trim().to_string();

    print!("archive base url: ");
    io::stdout().flush().unwrap();
    stdin.read_line(&mut archive_base_url).unwrap();
    let trimmed_archive_base_url = archive_base_url.trim().to_string();

    match init_batch_config(trimmed_batch_id, trimmed_archive_base_url) {
        Ok(()) => println!("Created {}", config_file::BATCH_CONFIG_FILE),
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            eprintln!("{} already exists", config_file::BATCH_CONFIG_FILE);
            std::process::exit(1);
        },
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}
