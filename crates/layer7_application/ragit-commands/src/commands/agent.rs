
use std::path::PathBuf;


use clap::Args;

#[derive(Debug, Args)]
pub struct AgentCommand {
    #[clap(subcommand)]
    pub command: AgentSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum AgentSubcommand {
    /// Communicate with an agent
    Communicate(CommunicateCommand),
}

#[derive(Debug, Args)]
pub struct CommunicateCommand {
    /// The name of the agent
    #[clap(short, long)]
    pub agent_name: String,

    /// The message to send
    #[clap(short, long)]
    pub message: String,
}

impl AgentCommand {
    pub fn run(self) -> anyhow::Result<()> {
        match self.command {
            AgentSubcommand::Communicate(command) => {
                let comms_dir = PathBuf::from("comms");
                let agent_dir = comms_dir.join(&command.agent_name);
                let outbox_dir = agent_dir.join("outbox");
                let inbox_dir = agent_dir.join("inbox");

                std::fs::create_dir_all(&outbox_dir)?;
                std::fs::create_dir_all(&inbox_dir)?;

                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_millis();
                let message_path = outbox_dir.join(format!("{:012}_message.md", timestamp));

                std::fs::write(message_path, &command.message)?;

                println!("Message sent to agent '{}'", command.agent_name);

                Ok(())
            }
        }
    }
}
