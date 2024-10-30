use std::fs::canonicalize;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct DirPath(String);
impl FromStr for DirPath {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = canonicalize(s).map_err(|_| "Path does not exist")?;
        if !path.is_dir() {
            return Err(format!("{} is not a directory", s));
        }
        Ok(Self(path.to_string_lossy().to_string()))
    }
}
impl DirPath {
    pub fn str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct FilePath(String);
impl FromStr for FilePath {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = canonicalize(s).map_err(|_| "Path does not exist")?;
        if !path.is_file() {
            return Err(format!("{} is not a file", s));
        }
        Ok(Self(path.to_string_lossy().to_string()))
    }
}
impl FilePath {
    pub fn str(&self) -> &str {
        &self.0
    }
}
