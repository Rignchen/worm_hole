use clap::Parser;
use crate::{
    db::Database,
    error::WHResult,
};

#[derive(Parser, Debug)]
pub struct Init {}
impl Init {
    pub fn run(&self, database: &Database) -> WHResult<()> {
        database.init();
        let bash_init_code =
            "alias 'wh=worm_hole'
            __worm_hole_cd() {
                if [ -z \"$1\" ]
                then
                    cd $HOME
                else
                    CD=$(wh query $1) && \\builtin cd $CD
                fi
            }
            alias 'whcd=__worm_hole_cd'";
        println!("{}", bash_init_code);
        Ok(())
    }
}
