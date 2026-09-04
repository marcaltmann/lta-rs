use clap::{Command, Arg, ArgAction};
use serde::Deserialize;
use std::io;
use std::io::prelude::*;
use std::fs::File;

#[derive(Deserialize)]
struct BatchConfig {
    base_url: String,
    id: String,
    resources: Vec<String>,
}

struct Config {
    config_file: String,
    force: bool,
}

fn main() {
    let config: Config = get_config();

    println!("{}", config.config_file);
    println!("{}", config.force);

    load_batch_config(config.config_file).expect("File should be readable JSON file");
}

fn get_config() -> Config {
    let matches = Command::new("lta-rs")
        .author("Marc Altmann, marcaltmann@posteo.de")
        .version("1.0")
        .about("Long term archiving tool")
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

    Config {
        config_file: String::from(config_file),
        force: *force,
    }
}

fn load_batch_config(path: String) -> io::Result<()> {
    let mut f = File::open(path)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    // Parse the string of data into serde_json::Value.
    let bc: BatchConfig = toml::from_str(&buffer).unwrap();

    // Access parts of the data by indexing with square brackets.
    println!("base_url: {}, id: {}", bc.base_url, bc.id);

    let resources = bc.resources;
    for res in resources {
        println!("{}", res);
    }

    Ok(())
}
