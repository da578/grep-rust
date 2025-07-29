//! Defines the `Config` structure for `grep-rust`, which holds all
//! the parsed command-line arguments.
//!
//! This module leverages the `clap` crate to automatically parse
//! command-line inputs into a strongly-typed struct, making argument
//! handling robust and easy to manage.

use clap::{Parser, arg, command};

/// Represents the configuration for the `grep-rust` application, derived
/// directly from command-line arguments.
///
/// This structure holds all parameters necessary for the search operation,
/// including the query string, file path, and various search options
/// like case-insensitivity, line numbering, and context lines.
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "This program is a basic implementation of the 'grep' utility. It allows users to search for a specific text pattern within a given file. Features include case-insensitive search, line numbering, and printing lines before/after a match (context)."
)]

pub struct Config {
    /// The string pattern to search for within the specified file.
    pub query: String,

    /// The path to the file where the search operation will be performed.
    pub file_path: String,

    /// Flag to enable case-insensitive searching. If set, the search
    /// will ignore differences in letter casing.
    #[arg(short, long)]
    pub ignore_case: bool,

    /// Flag to enable line numbering in the output. If set, each matching
    /// line (and its context) will be prefixed with its line number in the file.
    #[arg(short, long)]
    pub line_number: bool,

    /// Flag to enable word-only matching. The pattern will only match
    /// if it forms a whole word (bounded by non-word characters or)
    /// start/end of line.
    #[arg(short, long)]
    pub word_regexp: bool,

    /// Specifies the number of lines to print before a matching line.
    /// This provides "leading context" for matches. If not specified, defaults to 0.
    #[arg(short = 'B', long, value_name = "NUM")]
    pub before_context: Option<usize>,

    /// Specifies the number of lines to print after a matching line.
    /// This provides "trailing context" for matches. If not specified, defaults to 0.
    #[arg(short = 'A', long, value_name = "NUM")]
    pub after_context: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        // clap handles parsing, so we're primarily testing if our struct
        // correctly captures what clap would provide.
        // This test would typically be more complex if Config had custom
        // parsing or default logic beyond what clap provides.
        // For now, we just ensure it compiles and can be created.
        let args = vec!["grep-rust", "test_query", "test_file.txt"];
        let config = Config::parse_from(args);
        assert_eq!(config.query, "test_query");
        assert_eq!(config.file_path, "test_file.txt");
        assert!(!config.ignore_case);
        assert!(!config.line_number);
        assert_eq!(config.before_context, None);
        assert_eq!(config.after_context, None);
    }

    #[test]
    fn test_config_with_flags() {
        let args = vec![
            "grep-rust",
            "-i",
            "-l",
            "-B",
            "2",
            "-A",
            "3",
            "pattern",
            "file.log",
        ];
        let config = Config::parse_from(args);
        assert_eq!(config.query, "pattern");
        assert_eq!(config.file_path, "file.log");
        assert!(config.ignore_case);
        assert!(config.line_number);
        assert_eq!(config.before_context, Some(2));
        assert_eq!(config.after_context, Some(3));
    }

    #[test]
    fn test_config_with_word_regexp() {
        let args = vec!["grep-rust", "-w", "word", "file.txt"];
        let config = Config::parse_from(args);
        assert!(config.word_regexp);
        assert_eq!(config.query, "word");
    }
}
