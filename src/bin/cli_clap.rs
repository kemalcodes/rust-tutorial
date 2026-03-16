// Rust Tutorial #23: CLI Tools with Clap
// Demonstrates derive macro, positional args, flags, subcommands, and validation.

use clap::{Parser, Subcommand, Args, ValueEnum};
use std::path::PathBuf;

// ========================================================
// Basic CLI with Derive Macro
// ========================================================

/// A simple file utility tool
#[derive(Parser, Debug)]
#[command(name = "filetool")]
#[command(version = "1.0")]
#[command(about = "A file utility tool for demonstrations")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
}

// ========================================================
// Subcommands
// ========================================================

#[derive(Subcommand, Debug, PartialEq)]
enum Commands {
    /// Count lines, words, or characters in a file
    Count(CountArgs),

    /// Search for a pattern in a file
    Search(SearchArgs),

    /// Convert file to a different format
    Convert(ConvertArgs),

    /// Show file information
    Info {
        /// Path to the file
        path: PathBuf,
    },
}

// ========================================================
// Subcommand Arguments
// ========================================================

#[derive(Args, Debug, PartialEq)]
struct CountArgs {
    /// Path to the file to count
    path: PathBuf,

    /// What to count
    #[arg(short, long, default_value = "lines")]
    mode: CountMode,

    /// Show detailed breakdown
    #[arg(short, long)]
    detailed: bool,
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
enum CountMode {
    Lines,
    Words,
    Chars,
    All,
}

#[derive(Args, Debug, PartialEq)]
struct SearchArgs {
    /// The pattern to search for
    pattern: String,

    /// Path to the file to search
    path: PathBuf,

    /// Case-insensitive search
    #[arg(short, long)]
    ignore_case: bool,

    /// Maximum number of matches to show
    #[arg(short, long, default_value = "10")]
    max_results: usize,
}

#[derive(Args, Debug, PartialEq)]
struct ConvertArgs {
    /// Input file path
    input: PathBuf,

    /// Output file path
    output: PathBuf,

    /// Output format
    #[arg(short, long)]
    format: OutputFormat,
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
enum OutputFormat {
    Uppercase,
    Lowercase,
    Reversed,
}

// ========================================================
// Processing Functions
// ========================================================

fn count_content(text: &str, mode: &CountMode, detailed: bool) -> String {
    match mode {
        CountMode::Lines => {
            let count = text.lines().count();
            if detailed {
                let empty = text.lines().filter(|l| l.trim().is_empty()).count();
                format!("Total lines: {}\nEmpty lines: {}\nNon-empty lines: {}", count, empty, count - empty)
            } else {
                format!("{} lines", count)
            }
        }
        CountMode::Words => {
            let count = text.split_whitespace().count();
            if detailed {
                let unique: std::collections::HashSet<&str> = text.split_whitespace().collect();
                format!("Total words: {}\nUnique words: {}", count, unique.len())
            } else {
                format!("{} words", count)
            }
        }
        CountMode::Chars => {
            let count = text.chars().count();
            if detailed {
                let alpha = text.chars().filter(|c| c.is_alphabetic()).count();
                let digits = text.chars().filter(|c| c.is_numeric()).count();
                let spaces = text.chars().filter(|c| c.is_whitespace()).count();
                format!(
                    "Total chars: {}\nAlphabetic: {}\nDigits: {}\nWhitespace: {}",
                    count, alpha, digits, spaces
                )
            } else {
                format!("{} characters", count)
            }
        }
        CountMode::All => {
            let lines = text.lines().count();
            let words = text.split_whitespace().count();
            let chars = text.chars().count();
            format!("{} lines, {} words, {} characters", lines, words, chars)
        }
    }
}

fn search_content(text: &str, pattern: &str, ignore_case: bool, max_results: usize) -> Vec<String> {
    let mut results = Vec::new();

    for (i, line) in text.lines().enumerate() {
        let matches = if ignore_case {
            line.to_lowercase().contains(&pattern.to_lowercase())
        } else {
            line.contains(pattern)
        };

        if matches {
            results.push(format!("{}: {}", i + 1, line));
            if results.len() >= max_results {
                break;
            }
        }
    }

    results
}

fn convert_content(text: &str, format: &OutputFormat) -> String {
    match format {
        OutputFormat::Uppercase => text.to_uppercase(),
        OutputFormat::Lowercase => text.to_lowercase(),
        OutputFormat::Reversed => text.chars().rev().collect(),
    }
}

fn file_info(path: &PathBuf) -> String {
    let extension = path
        .extension()
        .map(|e| e.to_string_lossy().to_string())
        .unwrap_or_else(|| "none".to_string());

    let file_name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let parent = path
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| ".".to_string());

    format!(
        "File: {}\nExtension: {}\nDirectory: {}\nAbsolute: {}",
        file_name,
        extension,
        parent,
        path.is_absolute()
    )
}

// ========================================================
// A Simpler CLI Example (for the tutorial)
// ========================================================

/// A simple greeter CLI
#[derive(Parser, Debug)]
#[command(name = "greeter")]
#[command(about = "A simple greeter")]
struct SimpleGreeter {
    /// Name of the person to greet
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value = "1")]
    count: u32,

    /// Use uppercase greeting
    #[arg(short, long)]
    uppercase: bool,
}

fn greet(name: &str, count: u32, uppercase: bool) -> Vec<String> {
    let mut greetings = Vec::new();
    for i in 1..=count {
        let msg = format!("Hello, {}! (#{}/{})", name, i, count);
        if uppercase {
            greetings.push(msg.to_uppercase());
        } else {
            greetings.push(msg);
        }
    }
    greetings
}

// ========================================================
// Validation Example
// ========================================================

/// A CLI with validated arguments
#[derive(Parser, Debug)]
#[command(name = "validated")]
struct ValidatedCli {
    /// Port number (1024-65535)
    #[arg(short, long, value_parser = clap::value_parser!(u16).range(1024..=65535))]
    port: u16,

    /// Number of workers (1-32)
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=32))]
    workers: u8,

    /// Host address
    #[arg(long, default_value = "127.0.0.1")]
    host: String,
}

fn format_server_config(host: &str, port: u16, workers: u8) -> String {
    format!("Server: {}:{} with {} workers", host, port, workers)
}

fn main() {
    // In a real application, you would use:
    // let cli = Cli::parse();
    //
    // For this tutorial, we demonstrate parsing from specific arguments:

    println!("=== Clap CLI Demo ===\n");

    // Demo 1: Simple greeter
    println!("--- Simple Greeter ---");
    let greetings = greet("Alex", 3, false);
    for g in &greetings {
        println!("{}", g);
    }

    println!();

    // Demo 2: Count content
    println!("--- Count Demo ---");
    let sample_text = "Hello world\nRust is great\nClap makes CLIs easy\n\nThis is line five";
    let result = count_content(sample_text, &CountMode::All, false);
    println!("Content: {:?}", sample_text);
    println!("Result: {}", result);

    println!();

    // Demo 3: Detailed count
    println!("--- Detailed Count ---");
    let detailed = count_content(sample_text, &CountMode::Words, true);
    println!("{}", detailed);

    println!();

    // Demo 4: Search
    println!("--- Search Demo ---");
    let matches = search_content(sample_text, "is", false, 10);
    for m in &matches {
        println!("{}", m);
    }

    println!();

    // Demo 5: Case-insensitive search
    println!("--- Case-Insensitive Search ---");
    let text = "Rust is fast\nRUST is safe\nrust is fun";
    let matches = search_content(text, "rust", true, 10);
    for m in &matches {
        println!("{}", m);
    }

    println!();

    // Demo 6: Convert
    println!("--- Convert Demo ---");
    let original = "Hello, Rust World!";
    println!("Original: {}", original);
    println!("Uppercase: {}", convert_content(original, &OutputFormat::Uppercase));
    println!("Lowercase: {}", convert_content(original, &OutputFormat::Lowercase));
    println!("Reversed: {}", convert_content(original, &OutputFormat::Reversed));

    println!();

    // Demo 7: File info
    println!("--- File Info ---");
    let path = PathBuf::from("/home/user/documents/report.txt");
    println!("{}", file_info(&path));

    println!();

    // Demo 8: Server config
    println!("--- Validated Config ---");
    println!("{}", format_server_config("127.0.0.1", 8080, 4));

    println!("\n=== All demos completed! ===");
}

// ========================================================
// Tests
// ========================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_basic() {
        let result = greet("Sam", 1, false);
        assert_eq!(result, vec!["Hello, Sam! (#1/1)"]);
    }

    #[test]
    fn test_greet_multiple() {
        let result = greet("Alex", 3, false);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "Hello, Alex! (#1/3)");
        assert_eq!(result[2], "Hello, Alex! (#3/3)");
    }

    #[test]
    fn test_greet_uppercase() {
        let result = greet("Sam", 1, true);
        assert_eq!(result, vec!["HELLO, SAM! (#1/1)"]);
    }

    #[test]
    fn test_count_lines() {
        let text = "line one\nline two\nline three";
        let result = count_content(text, &CountMode::Lines, false);
        assert_eq!(result, "3 lines");
    }

    #[test]
    fn test_count_words() {
        let text = "hello world foo bar";
        let result = count_content(text, &CountMode::Words, false);
        assert_eq!(result, "4 words");
    }

    #[test]
    fn test_count_chars() {
        let text = "hello";
        let result = count_content(text, &CountMode::Chars, false);
        assert_eq!(result, "5 characters");
    }

    #[test]
    fn test_count_all() {
        let text = "hello world\nfoo bar";
        let result = count_content(text, &CountMode::All, false);
        assert_eq!(result, "2 lines, 4 words, 19 characters");
    }

    #[test]
    fn test_count_detailed_lines() {
        let text = "hello\n\nworld";
        let result = count_content(text, &CountMode::Lines, true);
        assert!(result.contains("Total lines: 3"));
        assert!(result.contains("Empty lines: 1"));
    }

    #[test]
    fn test_count_detailed_words() {
        let text = "hello hello world";
        let result = count_content(text, &CountMode::Words, true);
        assert!(result.contains("Total words: 3"));
        assert!(result.contains("Unique words: 2"));
    }

    #[test]
    fn test_count_detailed_chars() {
        let text = "ab1 2";
        let result = count_content(text, &CountMode::Chars, true);
        assert!(result.contains("Total chars: 5"));
        assert!(result.contains("Alphabetic: 2"));
        assert!(result.contains("Digits: 2"));
        assert!(result.contains("Whitespace: 1"));
    }

    #[test]
    fn test_search_basic() {
        let text = "hello world\nfoo bar\nhello again";
        let results = search_content(text, "hello", false, 10);
        assert_eq!(results.len(), 2);
        assert!(results[0].starts_with("1:"));
        assert!(results[1].starts_with("3:"));
    }

    #[test]
    fn test_search_case_insensitive() {
        let text = "Hello World\nhello world\nHELLO WORLD";
        let results = search_content(text, "hello", true, 10);
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_search_case_sensitive() {
        let text = "Hello World\nhello world\nHELLO WORLD";
        let results = search_content(text, "hello", false, 10);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_search_max_results() {
        let text = "aaa\naaa\naaa\naaa\naaa";
        let results = search_content(text, "aaa", false, 2);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_search_no_match() {
        let text = "hello world";
        let results = search_content(text, "xyz", false, 10);
        assert!(results.is_empty());
    }

    #[test]
    fn test_convert_uppercase() {
        assert_eq!(convert_content("hello", &OutputFormat::Uppercase), "HELLO");
    }

    #[test]
    fn test_convert_lowercase() {
        assert_eq!(convert_content("HELLO", &OutputFormat::Lowercase), "hello");
    }

    #[test]
    fn test_convert_reversed() {
        assert_eq!(convert_content("hello", &OutputFormat::Reversed), "olleh");
    }

    #[test]
    fn test_file_info_with_extension() {
        let path = PathBuf::from("/home/user/report.txt");
        let info = file_info(&path);
        assert!(info.contains("File: report.txt"));
        assert!(info.contains("Extension: txt"));
        assert!(info.contains("Directory: /home/user"));
    }

    #[test]
    fn test_file_info_no_extension() {
        let path = PathBuf::from("/home/user/Makefile");
        let info = file_info(&path);
        assert!(info.contains("Extension: none"));
    }

    #[test]
    fn test_file_info_relative_path() {
        let path = PathBuf::from("src/main.rs");
        let info = file_info(&path);
        assert!(info.contains("Absolute: false"));
    }

    #[test]
    fn test_format_server_config() {
        let result = format_server_config("localhost", 3000, 8);
        assert_eq!(result, "Server: localhost:3000 with 8 workers");
    }

    #[test]
    fn test_cli_parse_count_subcommand() {
        // Test that Clap can parse a count subcommand
        let cli = Cli::parse_from(["filetool", "count", "test.txt"]);
        assert!(!cli.verbose);
        match cli.command {
            Commands::Count(args) => {
                assert_eq!(args.path, PathBuf::from("test.txt"));
                assert!(!args.detailed);
            }
            _ => panic!("Expected Count command"),
        }
    }

    #[test]
    fn test_cli_parse_search_subcommand() {
        let cli = Cli::parse_from(["filetool", "search", "hello", "test.txt", "-i", "-m", "5"]);
        match cli.command {
            Commands::Search(args) => {
                assert_eq!(args.pattern, "hello");
                assert_eq!(args.path, PathBuf::from("test.txt"));
                assert!(args.ignore_case);
                assert_eq!(args.max_results, 5);
            }
            _ => panic!("Expected Search command"),
        }
    }

    #[test]
    fn test_cli_parse_convert_subcommand() {
        let cli = Cli::parse_from(["filetool", "convert", "in.txt", "out.txt", "-f", "uppercase"]);
        match cli.command {
            Commands::Convert(args) => {
                assert_eq!(args.input, PathBuf::from("in.txt"));
                assert_eq!(args.output, PathBuf::from("out.txt"));
                assert_eq!(args.format, OutputFormat::Uppercase);
            }
            _ => panic!("Expected Convert command"),
        }
    }

    #[test]
    fn test_cli_parse_verbose_flag() {
        let cli = Cli::parse_from(["filetool", "-v", "info", "test.txt"]);
        assert!(cli.verbose);
    }

    #[test]
    fn test_cli_parse_info_subcommand() {
        let cli = Cli::parse_from(["filetool", "info", "/path/to/file.rs"]);
        match cli.command {
            Commands::Info { path } => {
                assert_eq!(path, PathBuf::from("/path/to/file.rs"));
            }
            _ => panic!("Expected Info command"),
        }
    }

    #[test]
    fn test_simple_greeter_parse() {
        let cli = SimpleGreeter::parse_from(["greeter", "Alex", "-c", "5", "-u"]);
        assert_eq!(cli.name, "Alex");
        assert_eq!(cli.count, 5);
        assert!(cli.uppercase);
    }

    #[test]
    fn test_simple_greeter_defaults() {
        let cli = SimpleGreeter::parse_from(["greeter", "Sam"]);
        assert_eq!(cli.name, "Sam");
        assert_eq!(cli.count, 1);
        assert!(!cli.uppercase);
    }

    #[test]
    fn test_validated_cli_parse() {
        let cli = ValidatedCli::parse_from(["validated", "-p", "8080", "-w", "4"]);
        assert_eq!(cli.port, 8080);
        assert_eq!(cli.workers, 4);
        assert_eq!(cli.host, "127.0.0.1");
    }
}
