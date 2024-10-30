use crate::{db::Database, error::WHResult, path::FilePath};
use clap::Parser;
use string_builder::Builder as StringBuilder;
use std::str::FromStr;

#[derive(Parser, Debug)]
pub struct Init {
    /// The shell to generate the init code for
    shell: Shell,
    // All these flags will be used to set the commands to write in the console to call worm_hole
    // Only the worm_hole and cd command has a default value
    /// The command to use to call worm_hole
    #[clap(long, default_value = "wh")]
    worm_hole: String,
    /// The command to use to change directory with worm_hole
    #[clap(long, default_value = "whcd")]
    cd: String,
    /// The command to use to add a new path to worm_hole
    #[clap(long)]
    add: Option<String>,
    /// The command to use to remove a path from worm_hole
    #[clap(long)]
    remove: Option<String>,
    /// The command to use to list all the paths in worm_hole
    #[clap(long)]
    list: Option<String>,
    /// The command to use to query a path in worm_hole
    #[clap(long)]
    query: Option<String>,
    /// The command to use to edit a path in worm_hole
    #[clap(long)]
    edit: Option<String>,
    /// The command to use to rename an alias in worm_hole
    #[clap(long)]
    rename: Option<String>,
}
impl Init {
    pub fn run(&self, database: &Database, db_path: &str) -> WHResult<()> {
        database.init();
        println!("{}", self.shell.get_init_code(&self, db_path));
        Ok(())
    }
}

#[derive(clap::ValueEnum, Debug, Clone)]
enum Shell {
    Bash,
    Fish,
    Zsh,
}
impl Shell {
    pub fn get_init_code(&self, aliases: &Init, db_path: &str) -> String {
        let mut builder = StringBuilder::default();
        builder.append(format!("alias {}='worm_hole --db-path {}'\n", aliases.worm_hole, FilePath::from_str(db_path).unwrap().str()));
        builder.append(self.get_cd_function(aliases.worm_hole.as_str()));
        builder.append(format!("alias {}=__worm_hole_cd\n", aliases.cd));
        if let Some(add) = &aliases.add {
            builder.append(format!("alias {}='{} add'\n", add, aliases.worm_hole));
        }
        if let Some(remove) = &aliases.remove {
            builder.append(format!("alias {}='{} remove'\n", remove, aliases.worm_hole));
        }
        if let Some(list) = &aliases.list {
            builder.append(format!("alias {}='{} list'\n", list, aliases.worm_hole));
        }
        if let Some(query) = &aliases.query {
            builder.append(format!("alias {}='{} query'\n", query, aliases.worm_hole));
        }
        if let Some(edit) = &aliases.edit {
            builder.append(format!("alias {}='{} edit'\n", edit, aliases.worm_hole));
        }
        if let Some(rename) = &aliases.rename {
            builder.append(format!("alias {}='{} rename'\n", rename, aliases.worm_hole));
        }
        builder.string().unwrap()
    }

    fn get_cd_function(&self, worm_hole_command: &str) -> String {
        let mut builder = StringBuilder::default();
        match self {
            Shell::Bash | Shell::Zsh => {
                builder.append("__worm_hole_cd() {\n");
                builder.append("    if [ -z \"$1\" ]\n");
                builder.append("    then\n");
                builder.append("        cd $HOME\n");
                builder.append("    else\n");
                builder.append(format!("        CD=$({} query $1) && \\builtin cd $CD\n", worm_hole_command));
                builder.append("    fi\n");
                builder.append("}\n");
                builder.string().unwrap()
            }
            Shell::Fish => {
                builder.append("function __worm_hole_cd\n");
                builder.append("    if test -z $argv\n");
                builder.append("        cd $HOME\n");
                builder.append("    else\n");
                builder.append(format!("        set -l CD ({} query $argv)\n", worm_hole_command));
                builder.append("        if test -n $CD\n");
                builder.append("            builtin cd $CD\n");
                builder.append("        end\n");
                builder.append("    end\n");
                builder.append("end\n");
                builder.string().unwrap()
            }
        }
    }
}
