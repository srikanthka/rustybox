use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn run(file_path: &str){
    let path = Path::new(file_path);

    if !path.exists() || !path.is_file() {
        eprintln!("File not found:{}",path.display());
        return;
    }

    let content = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(err) => {
            eprintln!("Failed to read file: {}", err);
            return;
        }
    };

    let line_count = content.lines().count();
    let words: Vec<&str> = content
        .split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
        .filter(|w| !w.is_empty())
        .collect();

    let word_count = words.len();
    

    //Count frequencies
    let mut freq: HashMap<String, usize> = HashMap::new();

    for word in &words {
        let w = word.to_lowercase();
        *freq.entry(w).or_insert(0) += 1;
    }

    //Sort by frequency

    let mut top_words: Vec<(&String,&usize)> = freq.iter().collect();
    top_words.sort_by(|a,b|b.1.cmp(a.1));

    println!("File: {}", path.display());
    println!("Lines: {}", line_count);
    println!("Words: {}", word_count);
    println!("Top 10 words:");
    for(word,count) in top_words.iter().take(10) {
        println!("   - {}: {}", word, count);
    }
}
