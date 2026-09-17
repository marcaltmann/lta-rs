use clap::{Command, crate_name, crate_version, crate_description, crate_authors};
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use serde::Deserialize;

pub struct CLIParams {
    pub config_file: String,
    pub force: bool,
}

#[derive(Deserialize)]
pub struct BatchConfig {
    pub base_url: String,
    pub id: String,
    pub sessions: Vec<String>,
}

pub fn get_cli_params() -> CLIParams {
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
        .get_matches();

    let config_file = matches.get_one::<String>("config_file").unwrap();
    let force = matches.get_one::<bool>("force").unwrap();

    CLIParams {
        config_file: String::from(config_file),
        force: *force,
    }
}

pub fn load_batch_config(path: &Path) -> io::Result<BatchConfig> {
    let mut f = File::open(path)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    let config: BatchConfig = toml::from_str(&buffer).unwrap();

    Ok(config)
}
