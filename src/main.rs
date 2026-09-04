use clap::{Command, Arg, ArgAction};
use serde::{Deserialize, Serialize};
use std::io;
use std::io::prelude::*;
use std::fs::File;

#[derive(Serialize, Deserialize)]
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

    typed_example(config.config_file).expect("File should be readable JSON file");
}

fn get_config() -> Config {
    let matches = Command::new("lta-rs")
        .author("Marc Altmann, marcaltmann@posteo.de")
        .version("1.0")
        .about("Long term archiving tool")
        .arg(
            Arg::new("config_file")
                .required(true)
        )
        .arg(
            Arg::new("force")
                .short('f')
                .long("force")
                .help("Always fetch CMDI files")
                .action(ArgAction::SetTrue)
        )
        .after_help("Point the tool to the yaml configuration file \
                     in the batch directory.")
        .get_matches();

    let config_file: &String = matches.get_one::<String>("config_file").unwrap();
    let force: &bool = matches.get_one::<bool>("force").unwrap();

    return Config {
        config_file: String::from(config_file),
        force: *force,
    };
}

fn typed_example(path: String) -> io::Result<()> {
    let mut f = File::open(path)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    // Parse the string of data into serde_json::Value.
    let bc: BatchConfig = serde_json::from_str(&buffer)?;

    // Access parts of the data by indexing with square brackets.
    println!("base_url: {}, id: {}", bc.base_url, bc.id);

    let resources = bc.resources;
    for res in resources {
        println!("{}", res);
    }

    Ok(())
}
