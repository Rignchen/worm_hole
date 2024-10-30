use std::cmp::max;

pub struct Alias {
    alias: String,
    path: String,
}

impl From<(String, String)> for Alias {
    fn from((alias, path): (String, String)) -> Self {
        Alias { alias, path }
    }
}

impl Alias {
    pub fn len(&self) -> usize {
        self.alias.len()
    }
}

pub struct AliasList {
    aliases: Vec<Alias>,
    alias_max_len: usize,
}

impl AliasList {
    pub fn new() -> Self {
        AliasList {
            aliases: Vec::new(),
            alias_max_len: 0,
        }
    }

    pub fn add(&mut self, alias: Alias) {
        self.alias_max_len = max(self.alias_max_len, alias.len());
        self.aliases.push(alias);
    }

    pub fn print(&self) {
        for alias in &self.aliases {
            println!("{:width$} -> {}", alias.alias, alias.path, width = self.alias_max_len);
        }
    }
}
