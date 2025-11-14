use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct TopTermsArgs {
    #[clap(long)]
    pub count: Option<usize>,
    #[clap(long)]
    pub kb_path: Option<String>,
}
