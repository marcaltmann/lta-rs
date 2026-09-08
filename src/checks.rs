use std::io::{Error, ErrorKind};
use std::path::Path;
use std::fs;

pub fn check_dirs(sessions: &Vec<String>) -> Vec<Error> {
	let mut errors: Vec<Error> = Vec::new();

	for session in sessions {
		let path = Path::new(session);

		if !fs::exists(path).unwrap() {
			let message = format!("{} not found", session);
			let error = Error::new(ErrorKind::NotFound, message);
			errors.push(error);
			continue;
		}

		let metadata = path.metadata().unwrap();

		if !metadata.is_dir() {
			let message = format!("{} is no directory", session);
			let error = Error::new(ErrorKind::NotADirectory, message);
			errors.push(error);
			continue;
		}
	}

	errors
}
