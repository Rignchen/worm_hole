use crate::{db::Database, error::WHResult};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Init {
    /// The shell to generate the init code for
    shell: Shell,
}
impl Init {
    pub fn run(&self, database: &Database) -> WHResult<()> {
        database.init();
        println!("{}", self.shell.get_init_code());
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
    pub fn get_init_code(&self) -> &'static str {
        match self {
            Shell::Bash | Shell::Zsh => {
                "alias 'wh=worm_hole'
                __worm_hole_cd() {
                    if [ -z \"$1\" ]
                    then
                        cd $HOME
                    else
                        CD=$(wh query $1) && \\builtin cd $CD
                    fi
                }
                alias 'whcd=__worm_hole_cd'"
            }
            Shell::Fish => {
                "alias 'wh=worm_hole'
                function __worm_hole_cd
                    if test -z $argv
                        cd $HOME
                    else
                        set -l CD (wh query $argv)
                        if test -n $CD
                            builtin cd $CD
                        end
                    end
                end
                alias 'whcd=__worm_hole_cd'"
            }
        }
    }
}
