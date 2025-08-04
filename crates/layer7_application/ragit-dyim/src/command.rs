use anyhow::Result;
use clap::Subcommand;
use crate::dyim_command::dyim_command;

#[derive(Debug, Subcommand)]
pub enum DyimCommand {
    /// Interpret free-form text to generate prompts
    #[command(external_subcommand)]
    Dyim(Vec<String>),
}

impl DyimCommand {
    pub async fn run(self) -> Result<()> {
        match self {
            DyimCommand::Dyim(args) => {
                let input = args.join(" ");
                dyim_command(input).await
            }
        }
    }
}

