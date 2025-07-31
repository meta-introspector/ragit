use clap::Args;

#[derive(Args, Debug)]
pub struct SearchArgs {
    #[arg(help = "The regular expression pattern to search for.")]
    pub pattern: String,
    #[arg(long, help = "A glob pattern to filter which files are searched.")]
    pub include: Option<String>,
    #[arg(long, help = "The absolute path to the directory to search within.")]
    pub path: Option<String>,
}
