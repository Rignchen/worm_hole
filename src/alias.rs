struct Alias {
    alias: String,
    path: String,
}

impl From<(String, String)> for Alias {
    fn from((alias, path): (String, String)) -> Self {
        Alias { alias, path }
    }
}

pub struct AliasList {
    aliases: Vec<Alias>,
    alias_max_len: usize,
}

impl From<Vec<(String, String)>> for AliasList {
    fn from(aliases: Vec<(String, String)>) -> Self {
        AliasList {
            aliases: aliases.clone().into_iter().map(Alias::from).collect(),
            alias_max_len: aliases.iter().map(|(alias, _)| alias.len()).max().unwrap_or(0),
        }
    }
}

impl AliasList {
    pub fn print(&self) {
        for alias in &self.aliases {
            println!("{:width$} -> {}", alias.alias, alias.path, width = self.alias_max_len);
        }
    }
}
