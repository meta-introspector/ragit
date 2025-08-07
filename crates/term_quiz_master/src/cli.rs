use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The term to classify. If not provided, a random unclassified term will be displayed.
    #[clap(short, long)]
    pub term: Option<String>,

    /// The category for the term.
    #[clap(short, long)]
    pub category: Option<String>,

    /// The significance of the term.
    #[clap(short, long)]
    pub significance: Option<String>,

    /// The vibe of the term.
    #[clap(short, long)]
    pub vibe: Option<String>,

    /// An action suggestion for the term.
    #[clap(short, long)]
    pub action_suggestion: Option<String>,

    /// Run the update command to scan for Git changes and update terms.
    #[clap(long)]
    pub update: bool,

    /// Recurse into Git submodules when updating.
    #[clap(long)]
    pub recurse_submodules: bool,

    /// Perform a full depth clone/history traversal for Git operations.
    #[clap(long)]
    pub depth_full: bool,

    /// Start processing from this commit hash (defaults to genesis if not provided).
    #[clap(long)]
    pub start_commit: Option<String>,

    /// Number of commits to process per page before stopping.
    #[clap(long)]
    pub commits_per_page: Option<usize>,

    /// Do not recurse into Git submodules when updating.
    #[clap(long)]
    pub no_submodules: bool,
}
