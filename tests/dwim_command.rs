
use anyhow::Result;
use ragit_dwim::dwim_command;

#[tokio::test]
async fn test_dwim_command() -> Result<()> {
    let input = "test input".to_string();
    dwim_command(input).await?;
    Ok(())
}
