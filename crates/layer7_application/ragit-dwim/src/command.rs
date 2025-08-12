use anyhow::Result;
use clap::Subcommand;
use crate::dwim_command::dwim_command;

#[derive(Debug, Subcommand)]
pub enum DwimCommand {
    /// Interpret free-form text to generate prompts
    #[command(external_subcommand)]
    Dwim(Vec<String>),
}

impl DwimCommand {
    pub async fn run(self) -> Result<()> {
        match self {
            DwimCommand::Dwim(args) => {
                let input = args.join(" ");
                dwim_command(input).await
            }
        }
    }
}

