use clap::{Command, Arg};

fn main() {
    let matches = Command::new("lta-rs")
        .version("1.0")
        .about("Long term archiving tool")
        .arg(Arg::new("domain").short('d').long("domain").required(true))
        .arg(Arg::new("target").short('t').long("target").required(true))
        .get_matches();

    let domain: &String = matches.get_one::<String>("domain").unwrap();
    println!("{}", domain);
    let target_dir: &String = matches.get_one::<String>("target").unwrap();
    println!("{}", target_dir);
}
