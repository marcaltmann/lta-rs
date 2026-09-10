use std::path::Path;

mod checks;
use crate::checks::check_dirs;

mod config;
use crate::config::{get_cli_params, load_batch_config};

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
    for session in &config.sessions {
        println!("{}", session);
    }


    let parent = config_path.parent().unwrap();

    let f = |s| parent.join(s);
    let session_paths: Vec<_> = (&config).sessions.iter().map(f).collect();

    let errors = check_dirs(&session_paths);

    if !errors.is_empty() {
        println!("The following {} errors were found:", errors.len());
        for error in errors {
            println!("{:?}", error);
        }
    }
}
