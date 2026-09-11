use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::fs;

pub fn check_session_dirs(paths: &Vec<PathBuf>) -> Vec<Error> {
	let mut errors: Vec<Error> = Vec::new();

	for path in paths {
		let errors_for_round: Vec<Error> = check_directory(path.to_path_buf());

		for error in errors_for_round {
			errors.push(error);
		}
	}

	errors
}

pub fn check_directory(path: PathBuf) -> Vec<Error> {
	let mut errors: Vec<Error> = Vec::new();

	if !fs::exists(&path).unwrap() {
		let message = format!("{} not found", path.display());
		let error = Error::new(ErrorKind::NotFound, message);
		errors.push(error);
		return errors
	}

	let metadata = path.metadata().unwrap();

	if !metadata.is_dir() {
		let message = format!("{} is no directory", path.display());
		let error = Error::new(ErrorKind::NotADirectory, message);
		errors.push(error);
	}

	errors
}
