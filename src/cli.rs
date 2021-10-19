use super::*;
use crate::conf::Conf;
use crate::s3::daemon::Daemon;
use clap::{AppSettings, Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[clap(name = "s3d")]
#[clap(about = "s3d is an S3 daemon for the Edge written in Rust.")]
#[clap(setting = AppSettings::UseLongFormatForHelpSubcommand)]
#[clap(setting = AppSettings::DeriveDisplayOrder)]
pub struct CLI {
    /// Sets a custom config file path
    #[clap(long, short, name = "path")]
    config: Option<String>,

    /// Verbosity level, can be used multiple times
    #[clap(long, short, parse(from_occurrences))]
    verbose: i32,

    /// subcommands
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug, Clone)]
enum Cmd {
    Daemon(DaemonCmd),
    Fetch(FetchCmd),
    Pull(PullCmd),
    Status(StatusCmd),
    Diff(DiffCmd),
    Log(LogCmd),
    Push(PushCmd),
    Prune(PruneCmd),
    Get(GetCmd),
    Put(PutCmd),
    List(ListCmd),
}

impl CLI {
    pub async fn run() -> ResultOrAnyErr<()> {
        let cli: CLI = CLI::parse();
        println!("{:?}", cli);
        let conf = cli.load_conf().await?;
        match cli.cmd {
            Cmd::Daemon(cmd) => cmd.run(conf).await?,
            Cmd::Fetch(cmd) => cmd.run(conf).await?,
            Cmd::Pull(cmd) => cmd.run(conf).await?,
            Cmd::Status(cmd) => cmd.run(conf).await?,
            Cmd::Diff(cmd) => cmd.run(conf).await?,
            Cmd::Log(cmd) => cmd.run(conf).await?,
            Cmd::Push(cmd) => cmd.run(conf).await?,
            Cmd::Prune(cmd) => cmd.run(conf).await?,
            Cmd::Get(cmd) => cmd.run(conf).await?,
            Cmd::Put(cmd) => cmd.run(conf).await?,
            Cmd::List(cmd) => cmd.run(conf).await?,
        };
        Ok(())
    }

    async fn load_conf(&self) -> ResultOrAnyErr<Conf> {
        let mut conf = if let Some(ref config) = self.config {
            Conf::load(&config, true).await?
        } else {
            Conf::load("s3d.yaml", true).await?
        };
        // TODO: apply flags to conf
        // conf.verbose = self.verbose;
        conf.kind = String::from("s3d");
        println!("{:?}", conf);
        Ok(conf)
    }
}

#[derive(Parser, Debug, Clone)]
#[clap(about = "Run the daemon")]
pub struct DaemonCmd {
    #[clap(long, short)]
    debug: bool,
}

impl DaemonCmd {
    pub async fn run(self, conf: Conf) -> ResultOrAnyErr<()> {
        Daemon::run(conf).await
    }
}

#[derive(Parser, Debug, Clone, Copy)]
#[clap(about = "Fetch metadata from remote")]
struct FetchCmd {}

impl FetchCmd {
    pub async fn run(self, _conf: Conf) -> ResultOrAnyErr<()> {
        todo!();
        // Ok(())
    }
}

#[derive(Parser, Debug, Clone, Copy)]
#[clap(about = "Pull data and metadata from remote")]
struct PullCmd {}

impl PullCmd {
    pub async fn run(self, _conf: Conf) -> ResultOrAnyErr<()> {
        todo!();
        // Ok(())
    }
}

#[derive(Parser, Debug, Clone, Copy)]
#[clap(about = "...")]
struct StatusCmd {}

impl StatusCmd {
    pub async fn run(self, _conf: Conf) -> ResultOrAnyErr<()> {
        todo!();
        // Ok(())
    }
}

#[derive(Parser, Debug, Clone, Copy)]
#[clap(about = "...")]
struct DiffCmd {}

impl DiffCmd {
    pub async fn run(self, _conf: Conf) -> ResultOrAnyErr<()> {
        todo!();
        // Ok(())
    }
}

#[derive(Parser, Debug, Clone, Copy)]
#[clap(about = "...")]
struct LogCmd {}

impl LogCmd {
    pub async fn run(self, _conf: Conf) -> ResultOrAnyErr<()> {
        todo!();
        // Ok(())
    }
}

#[derive(Parser, Debug, Clone, Copy)]
#[clap(about = "...")]
struct PushCmd {}

impl PushCmd {
    pub async fn run(self, _conf: Conf) -> ResultOrAnyErr<()> {
        todo!();
        // Ok(())
    }
}

#[derive(Parser, Debug, Clone, Copy)]
#[clap(about = "...")]
struct PruneCmd {}

impl PruneCmd {
    pub async fn run(self, _conf: Conf) -> ResultOrAnyErr<()> {
        todo!();
        // Ok(())
    }
}

#[derive(Parser, Debug, Clone, Copy)]
#[clap(about = "...")]
struct GetCmd {}

impl GetCmd {
    pub async fn run(self, _conf: Conf) -> ResultOrAnyErr<()> {
        todo!();
        // Ok(())
    }
}

#[derive(Parser, Debug, Clone, Copy)]
#[clap(about = "...")]
struct PutCmd {}

impl PutCmd {
    pub async fn run(self, _conf: Conf) -> ResultOrAnyErr<()> {
        todo!();
        // Ok(())
    }
}

#[derive(Parser, Debug, Clone, Copy)]
#[clap(about = "...")]
struct ListCmd {}

impl ListCmd {
    pub async fn run(self, _conf: Conf) -> ResultOrAnyErr<()> {
        todo!();
        // Ok(())
    }
}
