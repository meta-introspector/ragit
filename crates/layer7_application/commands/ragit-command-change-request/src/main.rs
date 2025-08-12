use anyhow::Result;
use clap::Parser;
use ragit_qa_qm::change_management;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Description of the change request
    #[arg(short, long)]
    description: String,

    /// Name of the person requesting the change
    #[arg(short, long)]
    requested_by: String,

    /// Optional: Git branch name associated with the change
    #[arg(short, long)]
    branch_name: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let new_change = change_management::create_change(
        args.description,
        args.requested_by,
        args.branch_name,
    )?;;

    println!("Change Request Created: {:?}", new_change);

    Ok(())
}
