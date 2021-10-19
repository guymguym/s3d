use crate::{conf::Conf, util::*};
use clap::Parser;

#[derive(Parser, Debug, Clone, Copy)]
#[clap(about = "Status shows a summary of the local and remote buckets")]
pub struct StatusCmd {}

impl StatusCmd {
    pub async fn run(self, _conf: Conf) -> ResultOrAnyErr<()> {
        error!("TODO StatusCmd");
        Ok(())
    }
}
