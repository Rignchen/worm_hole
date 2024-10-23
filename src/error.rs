structstruck::strike! {
    /// List of errors that the program can return.
    #[strikethrough[derive(Debug)]]
    pub enum WHError {
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
                }
            );
            std::process::exit(1);
        }
    }
}

