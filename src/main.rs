use clap::{Command, Arg, ArgAction};

struct Config {
    config_file: String,
    force: bool,
}

fn main() {
    let config: Config = get_config();

    println!("{}", config.config_file);
    println!("{}", config.force);
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
