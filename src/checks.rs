use std::io::{Error, ErrorKind};
use std::path::PathBuf;

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

fn check_directory(path: PathBuf) -> Vec<Error> {
	let mut errors: Vec<Error> = Vec::new();

	let metadata_wrapper = path.metadata();

	let metadata = match metadata_wrapper {
		Ok(md) => md,
		Err(ref e)  if e.kind() == ErrorKind::NotFound => {
			let message = format!("{} not found", path.display());
            let error = Error::new(ErrorKind::NotFound, message);
			errors.push(error);
			return errors;
		},
        Err(e) => panic!("Another error occurred: {}", e),
	};

	if !metadata.is_dir() {
		let message = format!("{} is no directory", path.display());
		let error = Error::new(ErrorKind::NotADirectory, message);
		errors.push(error);
	}

	errors
}
