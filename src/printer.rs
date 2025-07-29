//! Provides utility functions for printing output in the `grep-rust` application.
//!
//! This module centralizes all display logic, including printing search
//! information and formatted output lines, ensuring consistent presentation.

use crate::config::Config;
use colored::*;
use std::fmt::Write;

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
    let mut output = String::new();

    output.push_str(
        format!(
            "Searching for '{}' in file '{}'...",
            config.query, config.file_path
        )
        .as_str(),
    );

    if config.ignore_case {
        output.push_str("\n(Case-insensitive search)");
    }
    if config.line_number {
        output.push_str("\n(Line numbers enabled)");
    }
    if before_context_num > 0 {
        output.push_str(format!("\n(Context before: {} lines)", before_context_num).as_str());
    }
    if after_context_num > 0 {
        output.push_str(format!("\n(Context after: {} lines)", after_context_num).as_str());
    }

    println!("{}", output.bold().yellow());
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
        print!("{}:  ", line_num.to_string().blue());
    }
    println!("{line_content}")
}

/// Prints a line of content, highlighting all occurrences of the search pattern
/// within that line. Optionally prefixes the line with its line number.
///
/// This function uses the provided `regex` to find all matches in `line_content`.
/// Each matched segment is formatted in green and bold, while the rest of the
/// line remains unformatted. Line numbers are printed in blue.
///
/// # Arguments
/// * `line_num` - The number of the line to print.
/// * `line_content` - The full string content of the line.
/// * `with_line_number` - A boolean flag indicating whether the line number
///                        should be included in the output.
/// * `regex` - A reference to the `regex::Regex` object used for matching.
///             This regex is used to find the exact positions of the pattern
///             within `line_content` for highlighting.
pub fn print_highlighted_line(
    line_num: usize,
    line_content: &str,
    with_line_num: bool,
    regex: &regex::Regex,
) {
    let mut output = String::new();
    let mut last_end = 0;

    if with_line_num {
        write!(&mut output, "{}:  ", line_num.to_string().blue()).unwrap();
    }

    // Iterate through all matches found by the regex in the line content.
    for m in regex.find_iter(line_content) {
        // Append the text before the current match.
        output.push_str(&line_content[last_end..m.start()]);

        // Append the matched text, formatted in green and bold.
        output.push_str(&format!(
            "{}",
            &line_content[m.start()..m.end()].green().bold()
        ));
        last_end = m.end();
    }

    // Append any remaining text after the last match.
    output.push_str(&line_content[last_end..]);

    println!("{}", output);
}
