extern crate s3d;
use clap::{CommandFactory, Parser};
use std::fmt::Debug;

/// main of the s3 binary
///
/// logging: by default only error! level is logged but it can be changed with env 
///     e.g RUST_LOG=trace or RUST_LOG=info,s3d=debug etc.
///
/// Parse the command line args using clap and run the requested subcommand.
#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    // env_logger::init();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("s3d=info"));

    let cli = CLI::parse();
    log::trace!("parsed command line args: {:?}", cli);

    match cli.cmd {
        Cmd::Api(cmd) => cmd.run().await,
        Cmd::Tag(cmd) => cmd.run().await,
        Cmd::Ls(cmd) => cmd.run().await,
        Cmd::Get(cmd) => cmd.run().await,
        Cmd::Put(cmd) => cmd.run().await,
        Cmd::Completion(cmd) => cmd.run(CLI::command()).await,
    }
}

/// CLI is the root command of cli.
/// Uses clap (derive style) to parse the program args into flags and subcommands.
#[derive(Parser, Debug, Clone)]
#[clap(name = "s3")]
#[clap(
    about = "S3 CLI tool for applications or services that need to access S3 buckets (with/out the s3d daemon)"
)]
#[clap(version = clap::crate_version!())]
#[clap(next_display_order = 0)]
pub struct CLI {
    /// subcommand
    #[clap(subcommand)]
    pub cmd: Cmd,
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum Cmd {
    // s3-style
    Api(s3d::cli::api_cmd::ApiCmd),
    Tag(s3d::cli::tag_cmd::TagCmd),
    // Id(s3d::cli::id_cmd::IdCmd),

    // fs-style
    Ls(s3d::cli::ls_cmd::LsCmd),
    // Cd(s3d::cli::cd_cmd::CdCmd),
    // Cp((s3d::cli::cp_cmd::CpCmd),
    // Rm((s3d::cli::rm_cmd::RmCmd),

    // kv-style
    Get(s3d::cli::get_cmd::GetCmd),
    Put(s3d::cli::put_cmd::PutCmd),
    // Del(s3d::cli::del_cmd::DelCmd),

    // git-style
    // - Init sets up config and local store for the daemon
    // - Fetch metadata only from remote
    // - Pull changes from remote
    // - Push changes to remote
    // - Prune objects from local store
    // - Diff shows objects pending for pull/push
    //
    // Remote(s3d::cli::RemoteCmd),
    // Status(s3d::cli::StatusCmd),
    // Fetch(s3d::cli::FetchCmd),
    // Pull(s3d::cli::PullCmd),
    // Push(s3d::cli::PushCmd),
    // Prune(s3d::cli::PruneCmd),
    // Diff(s3d::cli::DiffCmd),

    // other
    Completion(s3d::cli::completion_cmd::CompletionCmd),
}
