/// Generates shell commpletion script
#[derive(clap::Parser, Debug, Clone)]
pub struct CompletionCmd {
    #[clap(value_enum)]
    shell: clap_complete::Shell,
}

impl CompletionCmd {
    pub async fn run(&self, mut cmd: clap::Command) -> anyhow::Result<()> {
        let name = cmd.get_name().to_owned();
        Ok(clap_complete::generate(
            self.shell,
            &mut cmd,
            name,
            &mut std::io::stdout(),
        ))
    }
}
