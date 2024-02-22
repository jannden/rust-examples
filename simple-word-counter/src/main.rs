use std::collections::HashMap;
use std::env;
use std::fs;

fn read_file_to_string(file_path: &str) -> Result<String, std::io::Error> {
    println!("Reading {}.", file_path);
    fs::read_to_string(file_path)
}

fn prepare_text(text: &str) -> Vec<String> {
    text.to_lowercase() // Convert the text to lowercase, creating a new String
        .split_whitespace() // Split the new String into words
        .map(|s| s.to_string()) // Convert each &str to a String
        .collect() // Collect these Strings into a Vec<String>
}

fn count_word_occurrences(words: Vec<String>) -> HashMap<String, u32> {
    let mut occurrences = HashMap::new();
    for word in words {
        let count = occurrences.entry(word.to_string()).or_insert(0);
        *count += 1;
    }
    occurrences
}

fn sort_word_counts(word_counts: HashMap<String, u32>) -> Vec<(String, u32)> {
    let mut counts: Vec<_> = word_counts.into_iter().collect();
    // Sort by count in descending order, then by word in ascending order
    counts.sort_unstable_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    counts
}

fn display_common_words(sorted_counts: Vec<(String, u32)>, n: usize) {
    println!("Top {} most common words:", n);
    for (word, count) in sorted_counts.into_iter().take(n) {
        println!("{:<20} {}", word, count);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    match read_file_to_string(file_path) {
        Ok(content) => {
            let words = prepare_text(&content);
            let word_counts = count_word_occurrences(words);
            let sorted_counts = sort_word_counts(word_counts);
            // Adjust the number of common words displayed as needed
            let top_n = 10;
            display_common_words(sorted_counts, top_n);
        }
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            std::process::exit(1);
        }
    }
}
