use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct BatchConfig {
    pub base_url: String,
    pub id: String,
    pub resources: Vec<String>,
}

pub fn load_batch_config(path: &Path) -> io::Result<BatchConfig> {
    let mut f = File::open(path)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    let config: BatchConfig = toml::from_str(&buffer).unwrap();

    Ok(config)
}
