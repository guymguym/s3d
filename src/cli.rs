use crate::{conf::Conf, daemon};
use anyhow::Context;
use clap::Parser;
use std::fmt::Debug;
use std::path::Path;

#[derive(Parser, Debug, Clone)]
#[clap(name = clap::crate_name!())]
// #[clap(about = clap::crate_description!())]
#[clap(about = "s3d is an S3 daemon for the Edge written in Rust (https://s3d.rs)")]
#[clap(setting = clap::AppSettings::UseLongFormatForHelpSubcommand)]
#[clap(setting = clap::AppSettings::DeriveDisplayOrder)]
pub struct CLI {
    /// Sets a custom working directory for the daemon
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
    Run(RunCmd),
    Init(InitCmd),
    Fetch(FetchCmd),
    Pull(PullCmd),
    Push(PushCmd),
    Prune(PruneCmd),
    Status(StatusCmd),
    Diff(DiffCmd),
    Get(GetCmd),
    Put(PutCmd),
    List(ListCmd),
}

#[derive(Parser, Debug, Clone, Copy)]
#[clap(about = "Run the daemon")]
struct RunCmd {}

#[derive(Parser, Debug, Clone, Copy)]
#[clap(about = "Init sets up config and local store for the daemon")]
struct InitCmd {}

#[derive(Parser, Debug, Clone, Copy)]
#[clap(about = "Fetch metadata only from remote")]
struct FetchCmd {}

#[derive(Parser, Debug, Clone, Copy)]
#[clap(about = "Pull changes from remote")]
struct PullCmd {}

#[derive(Parser, Debug, Clone, Copy)]
#[clap(about = "Push changes to remote")]
struct PushCmd {}

#[derive(Parser, Debug, Clone, Copy)]
#[clap(about = "Prune objects from local store")]
struct PruneCmd {}

#[derive(Parser, Debug, Clone, Copy)]
#[clap(about = "Status shows a summary of the local state")]
struct StatusCmd {}

#[derive(Parser, Debug, Clone, Copy)]
#[clap(about = "Diff shows objects pending for pull/push")]
struct DiffCmd {}

#[derive(Parser, Debug, Clone, Copy)]
#[clap(about = "Get object (pull if needed)")]
struct GetCmd {}

#[derive(Parser, Debug, Clone, Copy)]
#[clap(about = "Put object (push if needed)")]
struct PutCmd {}

#[derive(Parser, Debug, Clone, Copy)]
#[clap(about = "List objects (fetch if needed)")]
struct ListCmd {}

impl CLI {
    pub async fn run() -> anyhow::Result<()> {
        // init default log levels - override with RUST_LOG env var
        env_logger::init_from_env(env_logger::Env::default().default_filter_or("warn,s3d=debug"));

        // parse command line arguments
        let cli = CLI::parse();

        debug!("{:?}", cli);

        // load the config file
        let conf = cli.load_conf().await?;

        // dispatch the command
        match cli.cmd {
            Cmd::Run(_cmd) => daemon::run(conf).await,
            // Cmd::Init(cmd) => cmd.run(conf).await,
            // Cmd::Fetch(cmd) => cmd.run(conf).await,
            // Cmd::Pull(cmd) => cmd.run(conf).await,
            // Cmd::Push(cmd) => cmd.run(conf).await,
            // Cmd::Prune(cmd) => cmd.run(conf).await,
            // Cmd::Status(cmd) => cmd.run(conf).await,
            // Cmd::Diff(cmd) => cmd.run(conf).await,
            // Cmd::Get(cmd) => cmd.run(conf).await,
            // Cmd::Put(cmd) => cmd.run(conf).await,
            // Cmd::List(cmd) => cmd.run(conf).await,
            cmd => bail!("CLI command not yet implemented: {:?}", cmd),
        }
    }

    async fn load_conf(&self) -> anyhow::Result<Conf> {
        let conf_path = Path::new(&self.dir).join("config");
        let conf = Conf::load(&conf_path)
            .await
            .with_context(|| format!("Failed to load config file \"{}\"", conf_path.display()))?;

        info!("Loaded config file \"{}\"", conf_path.display());

        // TODO: apply args/env to conf
        // conf.s3d = String::from("config");
        // conf.verbose = self.verbose;

        debug!("{:?}", conf);
        Ok(conf)
    }
}
