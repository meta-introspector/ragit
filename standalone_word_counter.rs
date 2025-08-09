use std::env;
use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let file_path = if args.len() > 1 {
        &args[1]
    } else {
        eprintln!("Usage: cargo run --bin tmp_word_counter <file_path>");
        return Err("Missing file path argument".into());
    };

    println!("Processing word counts for: {}", file_path);

    let content = fs::read_to_string(file_path)?;

    let word_regex = Regex::new(r"[a-zA-Z0-9_]+").unwrap();
    let mut word_counts: HashMap<String, usize> = HashMap::new();

    for line in content.lines() {
        for mat in word_regex.find_iter(line) {
            let word = mat.as_str().to_lowercase();
            *word_counts.entry(word).or_insert(0) += 1;
        }
    }

    // Sort word counts and display top N
    let mut sorted_word_counts: Vec<(&String, &usize)> = word_counts.iter().collect();
    sorted_word_counts.sort_by(|a, b| b.1.cmp(a.1)); // Sort descending by count

    println!("\n📝 Word Frequency Analysis:");
    println!("┌───────────────────────────┬───────────┐");
    println!("│ Word                      │ Count     │");
    println!("├───────────────────────────┼───────────┤");

    let top_n = 50; // Display top 50 words
    for (word, count) in sorted_word_counts.iter().take(top_n) {
        println!("│ {:<25} │ {:<9} │", word, count);
    }
    println!("└───────────────────────────┴───────────┘");
    println!("Total unique words: {}", word_counts.len());

    Ok(())
}
