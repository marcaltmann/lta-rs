use clap::{Command, Arg, ArgAction, crate_name, crate_version, crate_description, crate_authors};
use serde::Deserialize;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

#[derive(Deserialize)]
struct BatchConfig {
    base_url: String,
    id: String,
    resources: Vec<String>,
}

struct CLIParams {
    config_file: String,
    force: bool,
}

fn main() {
    let params = get_cli_params();
    println!("CLI params:");
    println!("Batch config file: {}", params.config_file);
    println!("Force: {}\n", params.force);

    let config_path = Path::new(&params.config_file);
    let config = load_batch_config(config_path).expect("File should be readable JSON file");
    println!("Batch config file contents:");
    println!("base_url: {}", config.base_url);
    println!("id: {}", config.id);
    for res in config.resources {
        println!("{}", res);
    }
}

fn get_cli_params() -> CLIParams {
    let matches = Command::new(crate_name!())
        .author(crate_authors!("\n"))
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::new("config_file")
                .required(true)
                .help("Batch config file at the base of the batch directory")
        )
        .arg(
            Arg::new("force")
                .short('f')
                .long("force")
                .help("Always fetch CMDI files")
                .action(ArgAction::SetTrue)
        )
        .after_help("Point the tool to the TOML configuration file \
                     in the batch directory.")
        .get_matches();

    let config_file = matches.get_one::<String>("config_file").unwrap();
    let force = matches.get_one::<bool>("force").unwrap();

    CLIParams {
        config_file: String::from(config_file),
        force: *force,
    }
}

fn load_batch_config(path: &Path) -> io::Result<BatchConfig> {
    let mut f = File::open(path)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    let config: BatchConfig = toml::from_str(&buffer).unwrap();

    Ok(config)
}
