structstruck::strike! {
	/// List of errors that the program can return.
	#[strikethrough[derive(Debug)]]
	pub enum WHError {
		DatabaseConnectionError(String),
		AliasNotFound(String),
		AliasAlreadyExists(String),
		PathOfAliasNotExist(String, String),
		PatternNotMatch(String),
	}
}

/// Result type which wither take a type T or a WHError.
pub type WHResult<T> = std::result::Result<T, WHError>;
pub fn unwrap_worm_hole_error<T>(result: WHResult<T>) -> T {
	match result {
		Ok(value) => value,
		Err(error) => {
			eprintln!(
                "{}",
                match error {
                    WHError::DatabaseConnectionError(path) =>
                        format!("Error connecting to database at {}", path),
                    WHError::AliasNotFound(alias) => format!("Alias {} does not exist", alias),
                    WHError::AliasAlreadyExists(alias) => format!("Cannot create alias {} because it already exists", alias),
                    WHError::PathOfAliasNotExist(alias, path) =>
                        format!("The path {} does no longer exist\nRun `wormhole rm {}` to remove the alias or `wormhole edit {} <new_path>` to update the path", path, alias, alias),
					WHError::PatternNotMatch(pattern) => format!("The pattern {} does not match anything", pattern),
                }
            );
			std::process::exit(1);
		}
	}
}
