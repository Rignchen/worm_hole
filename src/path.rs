use std::fs::canonicalize;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Path(String);
impl FromStr for Path {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = canonicalize(s).map_err(|_| "Path does not exist")?;
        if !path.is_dir() {
            return Err(format!("{} is not a directory", s));
        }
        Ok(Self(path.to_string_lossy().to_string()))
    }
}
impl Path {
    pub fn str(&self) -> &str {
        &self.0
    }
}

