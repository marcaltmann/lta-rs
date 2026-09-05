use clap::{Command, Arg, ArgAction, crate_name, crate_version, crate_description, crate_authors};
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
    pub resources: Vec<String>,
}

pub fn get_cli_params() -> CLIParams {
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

pub fn load_batch_config(path: &Path) -> io::Result<BatchConfig> {
    let mut f = File::open(path)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    let config: BatchConfig = toml::from_str(&buffer).unwrap();

    Ok(config)
}
