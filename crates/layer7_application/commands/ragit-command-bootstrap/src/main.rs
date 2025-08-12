use ragit_command_bootstrap::bootstrap_command::bootstrap_index_self;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // TODO: Parse command-line arguments
    bootstrap_index_self(
        true,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        None,
    )
    .await
}