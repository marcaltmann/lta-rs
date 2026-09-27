use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use serde::{Serialize,Deserialize};

const BATCH_CONFIG_FILE: &str = "batch.toml";

#[derive(Serialize, Deserialize)]
pub struct BatchConfig {
    pub batch_id: String,
    pub archive_base_url: String,
    pub sessions: Vec<String>,
}

pub fn init_batch_config() -> io::Result<()> {
	let new_config = BatchConfig {
		archive_base_url: String::from(""),
		batch_id: String::from("test"),
		sessions: Vec::new(),
	};

	let mut file = File::create(BATCH_CONFIG_FILE)?;
	let serialized = toml::to_string_pretty(&new_config).unwrap();
	file.write_all(serialized.as_bytes())?;

	Ok(())
}

pub fn _load_batch_config(path: &Path) -> io::Result<BatchConfig> {
    let mut f = File::open(path)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    let config: BatchConfig = toml::from_str(&buffer).unwrap();

    Ok(config)
}
