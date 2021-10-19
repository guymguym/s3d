use crate::{conf::Conf, util::*};
use clap::Parser;

#[derive(Parser, Debug, Clone, Copy)]
#[clap(about = "Status shows a summary of the local and remote buckets")]
pub struct StatusCmd {}

impl StatusCmd {
    pub async fn run(self, conf: Conf) -> ResultOrAnyErr<()> {
        println!("");
        println!("s3d status:");
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
