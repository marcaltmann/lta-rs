use std::io;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::Path;
use serde::{Serialize,Deserialize};

pub const BATCH_CONFIG_FILE: &str = "batch.toml";

#[derive(Serialize, Deserialize)]
pub struct BatchConfig {
    pub batch_id: String,
    pub archive_base_url: String,
    pub sessions: Vec<String>,
}

pub fn init_batch_config() -> io::Result<()> {
	// Try to get dir names
	let mut dir_names: Vec<String> = Vec::new();

	for entry in fs::read_dir(".")? {
		let entry = entry?;
		if entry.file_type()?.is_dir() {
			let name = entry.file_name().to_string_lossy().into_owned();
			if !name.starts_with('.') {
				dir_names.push(name);
			}
		}
	}
	dir_names.sort();

	let new_config = BatchConfig {
		archive_base_url: String::from(""),
		batch_id: String::from("test"),
		sessions: dir_names,
	};

	let mut file = File::create_new(BATCH_CONFIG_FILE)?;
	let serialized = toml::to_string_pretty(&new_config).unwrap();
	let content = format!("# lta batch configuration\n# Created by `lta init`\n\n{serialized}");

	file.write_all(content.as_bytes())?;

	Ok(())
}

pub fn _load_batch_config(path: &Path) -> io::Result<BatchConfig> {
    let mut f = File::open(path)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    let config: BatchConfig = toml::from_str(&buffer).unwrap();

    Ok(config)
}
