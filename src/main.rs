use clap::Parser;
use serde_json::Value;
use std::io::{self, Read};

#[derive(Parser)]
#[command(name = "caveman-rs", version)]
struct Args {
    /// Input JSON file (read from stdin if not specified)
    #[clap(short, long)]
    input: Option<String>,
    /// Output JSON file (write to stdout if not specified)
    #[clap(short, long)]
    output: Option<String>,
}

fn compress_text(text: &str) -> String {
    // Simple stop-word removal for English
    let stopwords = [
        "a", "an", "the", "and", "or", "but", "is", "are", "was", "were",
        "be", "been", "being", "have", "has", "had", "do", "does", "did",
        "will", "would", "shall", "should", "can", "could", "may", "might",
        "must", "i", "you", "he", "she", "it", "we", "they", "my", "your",
        "his", "her", "its", "our", "their", "this", "that", "these", "those",
    ];

    text.split_whitespace()
        .filter(|word| {
            let clean_word = word.trim_matches(|c: char| !c.is_alphanumeric());
            !stopwords.contains(&clean_word.to_lowercase().as_str())
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // Read input
    let mut input_data: Value = if let Some(file) = args.input {
        let file_content = std::fs::read_to_string(file)?;
        serde_json::from_str(&file_content)?
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        serde_json::from_str(&buffer)?
    };

    // Apply compression to the "content" field if it exists and is a string
    if let Value::String(content) = &mut input_data["content"] {
        let compressed = compress_text(content);
        // Only replace if we actually removed something
        if compressed.len() < content.len() {
            *content = compressed;
        }
    }

    // Write output
    let output_string = serde_json::to_string_pretty(&input_data)?;
    if let Some(file) = args.output {
        std::fs::write(file, output_string)?;
    } else {
        println!("{}", output_string);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compress_removes_stopwords() {
        assert_eq!(
            compress_text("The quick brown fox jumps over the lazy dog"),
            "quick brown fox jumps over lazy dog"
        );
    }

    #[test]
    fn compress_empty_string() {
        assert_eq!(compress_text(""), "");
    }

    #[test]
    fn compress_all_stopwords() {
        assert_eq!(compress_text("a the an and or but"), "");
    }

    #[test]
    fn compress_single_non_stopword() {
        assert_eq!(compress_text("Hello"), "Hello");
    }

    #[test]
    fn compress_single_stopword() {
        assert_eq!(compress_text("a"), "");
    }

    #[test]
    fn compress_preserves_case_of_kept_words() {
        assert_eq!(compress_text("The Quick Brown"), "Quick Brown");
    }

    #[test]
    fn compress_strips_punctuation_for_matching() {
        assert_eq!(compress_text("The end. A start!"), "end. start!");
    }

    #[test]
    fn compress_unicode_passthrough() {
        assert_eq!(compress_text("The café était très bien"), "café était très bien");
    }

    #[test]
    fn compress_no_words_removed() {
        assert_eq!(compress_text("programming algorithms data"), "programming algorithms data");
    }

    #[test]
    fn compress_whitespace_normalization() {
        assert_eq!(compress_text("The   quick   brown"), "quick brown");
    }

    #[test]
    fn compress_mixed_stopwords_and_content() {
        // "She" matches "she" (case-insensitive), "is"/"and"/"they"/"are"/"but"/"we"/"are" are stopwords
        assert_eq!(
            compress_text("She is running and they are jumping but we are walking"),
            "running jumping walking"
        );
    }
}
