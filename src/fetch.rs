use crate::{conf::Conf, util::*};
use clap::Parser;

#[derive(Parser, Debug, Clone, Copy)]
#[clap(about = "Fetch metadata only from the remote bucket")]
pub struct FetchCmd {}

impl FetchCmd {
    pub async fn run(self, conf: Conf) -> ResultOrAnyErr<()> {
        println!("");
        println!("s3d fetch:");
        println!("");
        println!("Local {:#?}", conf.local);
        println!("");
        for remote in conf.remotes.iter() {
            println!("Remote {:#?}", remote);
            println!("");
        }
        Ok(())
    }
}
