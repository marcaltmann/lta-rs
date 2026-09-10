use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::fs;

pub fn check_dirs(paths: &Vec<PathBuf>) -> Vec<Error> {
	let mut errors: Vec<Error> = Vec::new();

	for path in paths {
		if !fs::exists(path).unwrap() {
			let message = format!("{} not found", path.display());
			let error = Error::new(ErrorKind::NotFound, message);
			errors.push(error);
			continue;
		}

		let metadata = path.metadata().unwrap();

		if !metadata.is_dir() {
			let message = format!("{} is no directory", path.display());
			let error = Error::new(ErrorKind::NotADirectory, message);
			errors.push(error);
			continue;
		}
	}

	errors
}
