use clap::{Command, Arg};

struct Config {
    domain: String,
    target_dir: String,
}

fn main() {
    let config: Config = get_config();

    println!("{}", config.domain);
    println!("{}", config.target_dir);
}

fn get_config() -> Config {
    let matches = Command::new("lta-rs")
        .version("1.0")
        .about("Long term archiving tool")
        .arg(Arg::new("domain").short('d').long("domain").required(true))
        .arg(Arg::new("target").short('t').long("target-dir").required(true))
        .get_matches();

    let domain: &String = matches.get_one::<String>("domain").unwrap();
    let target_dir: &String = matches.get_one::<String>("target").unwrap();

    return Config {
        domain: String::from(domain),
        target_dir: String::from(target_dir),
    };
}
