use crate::{db::Database, error::WHResult};
use clap::Parser;
use string_builder::Builder as StringBuilder;

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
}
impl Init {
    pub fn run(&self, database: &Database) -> WHResult<()> {
        database.init();
        println!("{}", self.shell.get_init_code(&self));
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
    pub fn get_init_code(&self, aliases: &Init) -> String {
        let mut builder = StringBuilder::default();
        builder.append(format!("alias {}=worm_hole\n", aliases.worm_hole));
        builder.append(self.get_cd_function());
        builder.append(format!("alias {}=__worm_hole_cd\n", aliases.cd));
        if let Some(add) = &aliases.add {
            builder.append(format!("alias {}='worm_hole add'\n", add));
        }
        if let Some(remove) = &aliases.remove {
            builder.append(format!("alias {}='worm_hole remove'\n", remove));
        }
        if let Some(list) = &aliases.list {
            builder.append(format!("alias {}='worm_hole list'\n", list));
        }
        if let Some(query) = &aliases.query {
            builder.append(format!("alias {}='worm_hole query'\n", query));
        }
        if let Some(edit) = &aliases.edit {
            builder.append(format!("alias {}='worm_hole edit'\n", edit));
        }
        builder.string().unwrap()
    }

    fn get_cd_function(&self) -> &'static str {
        match self {
            Shell::Bash | Shell::Zsh => {
                "__worm_hole_cd() {
                    if [ -z \"$1\" ]
                    then
                        cd $HOME
                    else
                        CD=$(wh query $1) && \\builtin cd $CD
                    fi
                }
                "
            }
            Shell::Fish => {
                "function __worm_hole_cd
                    if test -z $argv
                        cd $HOME
                    else
                        set -l CD (wh query $argv)
                        if test -n $CD
                            builtin cd $CD
                        end
                    end
                end
                "
            }
        }
    }
}
