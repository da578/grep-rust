//! Provides utility functions for printing output in the `grep-rust` application.
//!
//! This module centralizes all display logic, including printing search
//! information and formatted output lines, ensuring consistent presentation.

use crate::config::Config;

/// Prints the initial information about the search operation to the console.
///
/// This includes the query string, file path, and active search options
/// like case-insensitivity, line numbering, and context line counts.
///
/// # Arguments
/// * `config` - A reference to the `Config` struct containing search options.
/// * `before_context_num` - The number of lines of context to show before a match.
/// * `after_context_num` - The number of lines of context to show after a match.
pub fn print_search_info(config: &Config, before_context_num: usize, after_context_num: usize) {
    println!(
        "Searching for '{}' in file '{}'...",
        config.query, config.file_path
    );

    if config.ignore_case {
        println!("(Case-insensitive search)");
    }
    if config.line_number {
        println!("(Line numbers enabled)");
    }
    if before_context_num > 0 {
        println!("(Context before: {} lines)", before_context_num);
    }
    if after_context_num > 0 {
        println!("(Context after: {} lines)", after_context_num);
    }
}

/// Prints a single line of content, optionally prefixed with its line number.
///
/// This function handles the formatting of individual output lines based on
/// the `line_number` configuration.
///
/// # Arguments
/// * `line_num` - The number of the line to print.
/// * `line_content` - The string content of the line to print.
/// * `with_line_number` - A boolean flag indicating whether the line number
///                        should be included in the output.
pub fn print_line(line_num: usize, line_content: &str, with_line_number: bool) {
    if with_line_number {
        print!("{line_num}:");
    }
    println!("{line_content}")
}