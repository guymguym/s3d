pub mod daemon;
pub mod fetch;
pub mod status;

pub use self::daemon::DaemonCmd;
pub use self::fetch::FetchCmd;
pub use self::status::StatusCmd;

use crate::conf::Conf;
use crate::util::*;
use clap::Parser;

#[derive(Parser, Debug, Clone, Copy)]
#[clap(about = "Diff shows the list of objects that are pending for pull or push")]
pub struct DiffCmd {}

impl DiffCmd {
    pub async fn run(self, _conf: Conf) -> ResultOrAnyErr<()> {
        error!("TODO DiffCmd");
        Ok(())
    }
}

#[derive(Parser, Debug, Clone, Copy)]
#[clap(about = "Log shows the history of recent operations")]
pub struct LogCmd {}

impl LogCmd {
    pub async fn run(self, _conf: Conf) -> ResultOrAnyErr<()> {
        error!("TODO LogCmd");
        Ok(())
    }
}

#[derive(Parser, Debug, Clone, Copy)]
#[clap(about = "Pull changes from the remote bucket")]
pub struct PullCmd {}

impl PullCmd {
    pub async fn run(self, _conf: Conf) -> ResultOrAnyErr<()> {
        error!("TODO PullCmd");
        Ok(())
    }
}

#[derive(Parser, Debug, Clone, Copy)]
#[clap(about = "Push changes to the remote bucket")]
pub struct PushCmd {}

impl PushCmd {
    pub async fn run(self, _conf: Conf) -> ResultOrAnyErr<()> {
        error!("TODO PushCmd");
        Ok(())
    }
}

#[derive(Parser, Debug, Clone, Copy)]
#[clap(about = "Prune objects from local bucket")]
pub struct PruneCmd {}

impl PruneCmd {
    pub async fn run(self, _conf: Conf) -> ResultOrAnyErr<()> {
        error!("TODO PruneCmd");
        Ok(())
    }
}

#[derive(Parser, Debug, Clone, Copy)]
#[clap(
    about = "Put an object from the local bucket (optionally pull from remote bucket if missing or changed)"
)]
pub struct GetCmd {}

impl GetCmd {
    pub async fn run(self, _conf: Conf) -> ResultOrAnyErr<()> {
        error!("TODO GetCmd");
        Ok(())
    }
}

#[derive(Parser, Debug, Clone, Copy)]
#[clap(about = "Put an object to the local bucket (optionally push immediately to remote bucket)")]
pub struct PutCmd {}

impl PutCmd {
    pub async fn run(self, _conf: Conf) -> ResultOrAnyErr<()> {
        error!("TODO PutCmd");
        Ok(())
    }
}

#[derive(Parser, Debug, Clone, Copy)]
#[clap(
    about = "List bucket objects from local bucket (optionally fetch from remote bucket if needed)"
)]
pub struct ListCmd {}

impl ListCmd {
    pub async fn run(self, _conf: Conf) -> ResultOrAnyErr<()> {
        error!("TODO ListCmd");
        Ok(())
    }
}
