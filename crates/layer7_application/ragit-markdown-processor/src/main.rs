use clap::Parser;
use pulldown_cmark::{Parser as MarkdownParser, Options, html};
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input markdown file or directory containing markdown files
    #[clap(short, long, value_parser)]
    input: PathBuf,

    /// Output directory for processed HTML files
    #[clap(short, long, value_parser)]
    output: PathBuf,
}

fn process_markdown_file(input_path: &PathBuf, output_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let markdown_input = fs::read_to_string(input_path)?;

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);

    let parser = MarkdownParser::new_ext(&markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    let output_file_name = input_path.file_stem().unwrap().to_str().unwrap().to_owned() + ".html";
    let output_path = output_dir.join(output_file_name);

    fs::write(&output_path, html_output)?;
    println!("Processed {} to {}", input_path.display(), output_path.display());

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if !args.output.exists() {
        fs::create_dir_all(&args.output)?;
    }

    if args.input.is_dir() {
        for entry in fs::read_dir(&args.input)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                process_markdown_file(&path, &args.output)?;
            }
        }
    } else if args.input.is_file() && args.input.extension().map_or(false, |ext| ext == "md") {
        process_markdown_file(&args.input, &args.output)?;
    } else {
        eprintln!("Error: Input must be a markdown file or a directory containing markdown files.");
        std::process::exit(1);
    }

    Ok(())
}