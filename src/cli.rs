use super::*;
use crate::conf::Conf;
use crate::daemon::DaemonCmd;
use crate::fetch::FetchCmd;
use crate::status::StatusCmd;
use clap::Parser;
use std::path::Path;

#[derive(Parser, Debug, Clone)]
#[clap(name = clap::crate_name!())]
#[clap(about = clap::crate_description!())]
// #[clap(about = "s3d is an S3 daemon for the Edge written in Rust.")]
#[clap(setting = clap::AppSettings::UseLongFormatForHelpSubcommand)]
#[clap(setting = clap::AppSettings::DeriveDisplayOrder)]
pub struct CLI {
    /// Sets a custom config file path
    #[clap(long, short, name = "PATH", default_value = ".s3d")]
    dir: String,

    /// Verbosity level, can be used multiple times
    #[clap(long, short, parse(from_occurrences))]
    verbose: i32,

    /// subcommands
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(clap::Subcommand, Debug, Clone)]
enum Cmd {
    Daemon(DaemonCmd),
    Status(StatusCmd),
    Diff(DiffCmd),
    Log(LogCmd),
    Fetch(FetchCmd),
    Pull(PullCmd),
    Push(PushCmd),
    Prune(PruneCmd),
    Get(GetCmd),
    Put(PutCmd),
    List(ListCmd),
}

impl CLI {
    pub async fn run() -> ResultOrAnyErr<()> {
        // log all levels by default if the RUST_LOG environment variable isn’t set
        // env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();
        env_logger::init();

        // parse the command line arguments
        let cli = CLI::parse();
        debug!("{:?}", cli);

        // load the config file
        let conf = cli.load_conf().await?;

        // dispatch the command
        match cli.cmd {
            Cmd::Daemon(cmd) => cmd.run(conf).await?,
            Cmd::Status(cmd) => cmd.run(conf).await?,
            Cmd::Diff(cmd) => cmd.run(conf).await?,
            Cmd::Log(cmd) => cmd.run(conf).await?,
            Cmd::Fetch(cmd) => cmd.run(conf).await?,
            Cmd::Pull(cmd) => cmd.run(conf).await?,
            Cmd::Push(cmd) => cmd.run(conf).await?,
            Cmd::Prune(cmd) => cmd.run(conf).await?,
            Cmd::Get(cmd) => cmd.run(conf).await?,
            Cmd::Put(cmd) => cmd.run(conf).await?,
            Cmd::List(cmd) => cmd.run(conf).await?,
        };

        Ok(())
    }

    async fn load_conf(&self) -> ResultOrAnyErr<Conf> {
        let conf_path = Path::new(&self.dir).join("config");
        let mut conf = Conf::load(&conf_path).await.or_else(|err| {
            // translate the error to a meaningful message
            Err(format!(
                "Failed to load config file \"{}\" with error \"{}\"",
                conf_path.display(),
                err
            ))
        })?;
        info!("Loaded config file \"{}\"", conf_path.display());

        // TODO: apply args/env to conf
        conf.s3d = String::from("config");
        // conf.verbose = self.verbose;

        debug!("{:?}", conf);
        Ok(conf)
    }
}

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
