use crate::{conf::Conf, daemon};
use anyhow::Context;
use clap;
use clap::Parser;
use std::fmt::Debug;

/// s3d is an S3 daemon for the Edge written in Rust (https://s3d.rs)
#[derive(Parser, Debug, Clone)]
#[clap(name = "s3d")]
#[clap(setting = clap::AppSettings::UseLongFormatForHelpSubcommand)]
#[clap(setting = clap::AppSettings::DeriveDisplayOrder)]
// #[clap(name = clap::crate_name!())]
// #[clap(about = clap::crate_description!())]
pub struct CLI {
    /// Sets a custom working directory for the daemon
    // #[clap(long, short, name = "PATH", default_value = ".s3d")]
    // dir: String,

    /// Verbosity level, can be used multiple times
    // #[clap(long, short, parse(from_occurrences))]
    // verbose: i32,

    /// subcommand
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(clap::Subcommand, Debug, Clone)]
enum Cmd {
    Run(RunCmd),
    Status(StatusCmd),
    List(ListCmd),
    Get(GetCmd),
    Put(PutCmd),
    Tag(TagCmd),
}

/// Run the daemon
#[derive(Parser, Debug, Clone)]
struct RunCmd {}

/// Status of the daemon and the data
#[derive(Parser, Debug, Clone)]
struct StatusCmd {}

/// List buckets or objects
#[derive(Parser, Debug, Clone)]
#[clap(aliases = &["ls"])]
struct ListCmd {
    /// List objects in a bucket, otherwise list all buckets
    bucket: Option<String>,
    /// List objects with key prefix
    prefix: Option<String>,
}

/// Get object to stdout
#[derive(Parser, Debug, Clone)]
struct GetCmd {
    /// Get object from bucket
    bucket: String,
    /// Get object with key
    key: String,
}

/// Put object from stdin
#[derive(Parser, Debug, Clone)]
struct PutCmd {
    /// Put object in bucket
    bucket: String,
    /// Put object with key
    key: String,
}

/// Tag bucket/object
#[derive(Parser, Debug, Clone)]
struct TagCmd {
    /// Tag object in bucket
    bucket: String,
    /// Tag object with key
    key: String,
    /// Tag name
    tag: String,
    /// Tag value
    val: String,
    /// Reset existing tags
    #[clap(long, short)]
    reset: bool,
}

impl CLI {
    pub async fn run() -> anyhow::Result<()> {
        // init default log levels - override with RUST_LOG env var
        env_logger::init_from_env(env_logger::Env::default().default_filter_or("warn,s3d=info"));

        // parse command line arguments
        let cli = CLI::parse();

        debug!("{:?}", cli);

        // load the configuration
        let conf = cli.load_conf().await?;

        // dispatch the command
        match cli.cmd {
            Cmd::Run(_cmd) => daemon::run(conf).await,
            Cmd::Status(cmd) => cmd.run(conf).await,
            Cmd::List(cmd) => cmd.run(conf).await,
            Cmd::Get(cmd) => cmd.run(conf).await,
            Cmd::Put(cmd) => cmd.run(conf).await,
            Cmd::Tag(cmd) => cmd.run(conf).await,
            // Cmd::Init(cmd) => cmd.run(conf).await,
            // Cmd::Fetch(cmd) => cmd.run(conf).await,
            // Cmd::Pull(cmd) => cmd.run(conf).await,
            // Cmd::Push(cmd) => cmd.run(conf).await,
            // Cmd::Prune(cmd) => cmd.run(conf).await,
            // Cmd::Diff(cmd) => cmd.run(conf).await,
            // cmd => bail!("CLI command not yet implemented: {:?}", cmd),
        }
    }

    async fn load_conf(&self) -> anyhow::Result<Conf> {
        let conf = Conf::default();
        // let conf = Conf::load(&self.dir)
        //     .await
        //     .with_context(|| format!("Failed to load config file from dir \"{}\"", self.dir))?;

        // info!("Loaded config file from dir \"{}\"", self.dir);

        // TODO: apply args/env to conf
        // conf.s3d = String::from("config");
        // conf.verbose = self.verbose;

        debug!("{:?}", conf);
        Ok(conf)
    }
}

impl StatusCmd {
    async fn run(&self, _conf: Conf) -> anyhow::Result<()> {
        debug!("{:?}", self);
        Ok(())
    }
}
impl ListCmd {
    async fn run(&self, _conf: Conf) -> anyhow::Result<()> {
        debug!("{:?}", self);
        Ok(())
    }
}
impl GetCmd {
    async fn run(&self, _conf: Conf) -> anyhow::Result<()> {
        debug!("{:?}", self);
        Ok(())
    }
}
impl PutCmd {
    async fn run(&self, _conf: Conf) -> anyhow::Result<()> {
        debug!("{:?}", self);
        Ok(())
    }
}
impl TagCmd {
    async fn run(&self, _conf: Conf) -> anyhow::Result<()> {
        debug!("{:?}", self);
        Ok(())
    }
}

// enum Cmd {
//     ...
//     Init(InitCmd),
//     Fetch(FetchCmd),
//     Pull(PullCmd),
//     Push(PushCmd),
//     Prune(PruneCmd),
//     Diff(DiffCmd),
// }

// #[derive(Parser, Debug, Clone, Copy)]
// #[clap(about = "Init sets up config and local store for the daemon")]
// struct InitCmd {}

// #[derive(Parser, Debug, Clone, Copy)]
// #[clap(about = "Fetch metadata only from remote")]
// struct FetchCmd {}

// #[derive(Parser, Debug, Clone, Copy)]
// #[clap(about = "Pull changes from remote")]
// struct PullCmd {}

// #[derive(Parser, Debug, Clone, Copy)]
// #[clap(about = "Push changes to remote")]
// struct PushCmd {}

// #[derive(Parser, Debug, Clone, Copy)]
// #[clap(about = "Prune objects from local store")]
// struct PruneCmd {}

// #[derive(Parser, Debug, Clone, Copy)]
// #[clap(about = "Diff shows objects pending for pull/push")]
// struct DiffCmd {}
