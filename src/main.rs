use std::path::Path;

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
    for res in config.resources {
        println!("{}", res);
    }
}
