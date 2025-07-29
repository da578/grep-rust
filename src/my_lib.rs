//! Contains the core logic for the `grep-rust` application.
//!
//! This module defines the `GrepState` to manage the search process's internal
//! state and the `run` function, which orchestrates file reading, pattern
//! matching, and context handling.

use regex::Regex;

use crate::config::Config;
use crate::printer::{print_highlighted_line, print_line, print_search_info};

use std::{
    collections::VecDeque,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

/// Represents the mutable state of the grep operation as it processes lines.
///
/// This struct holds counters, buffers for context lines, and flags to
/// manage the output behavior during the search.
struct GrepState {
    /// The current line number being processed from the input file.
    line_count: usize,
    /// A buffer holding lines encountered before a match, used for `--before-context`.
    before_context_buffer: VecDeque<(usize, String)>,
    /// A counter indicating how many lines of "after context" still need to be printed.
    lines_after_match: usize,
    /// A flag indicating if the current output block is active (i.e., we are
    /// printing a match or its context lines). This helps manage context printing
    /// across consecutive matches
    printing_block_active: bool,
}

impl GrepState {
    /// Creates a new, initialized `GrepState` with default values.
    ///
    /// All counters are set to zero, buffers are empty, and printing is inactive.
    fn new() -> Self {
        GrepState {
            line_count: 0,
            before_context_buffer: VecDeque::new(),
            lines_after_match: 0,
            printing_block_active: false,
        }
    }
}

/// Executes the main grep search logic based on the provided configuration.
///
/// This function reads the specified file line by line, performs pattern
/// matching, and prints lines along with their before and after context
/// according to the `Config`.
///
/// # Arguments
/// * `config` - A `Config` struct containing all parsed command-line arguments
///              and search options.
///
/// # Returns
/// A `Result` indicating success (`Ok(())`) or an error (`Err(Box<dyn Error>)`)
/// if an issue occurs during file operations or other processes.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let before_context_num = config.before_context.unwrap_or(0);
    let after_context_num = config.after_context.unwrap_or(0);

    // Print initial search information using the printer module.
    print_search_info(&config, before_context_num, after_context_num);

    // Prepare the regex pattern string. If `word_regexp` is enabled,
    // word boundaries (`\b`) are added around the escaped query.
    let pattern_string = if config.word_regexp {
        format!(r"\b{}\b", regex::escape(&config.query))
    } else {
        regex::escape(&config.query)
    };

    let search_regex = if config.ignore_case {
        Regex::new(&format!("(?i){}", pattern_string))
            .map_err(|e| format!("Invalid regex pattern: {}", e))?
    } else {
        // If not case-insensitive, just use the built regex as is.
        Regex::new(&pattern_string).map_err(|e| format!("Invalid regex pattern: {}", e))?
    };

    // Open the file and create a buffered reader for efficient line-by-line reading.
    // The `?` operator handles potential file opening errors.
    let file = File::open(config.file_path)?;
    let reader = BufReader::new(file);

    let mut state = GrepState::new();

    // Iterate through each line of the file.
    for line_result in reader.lines() {
        state.line_count += 1; // Increment line count for each line processed
        let line = line_result?; // Get the current line content
        let current_line_ref = &line;

        // Check if the current line matches the processed query.
        // `find().is_some()` returns true if the regex finds at least one match.
        let is_match = search_regex.find(current_line_ref).is_some();

        // Use a match statement to handle different scenarios based on `is_match`
        // and whether we are currently printing "after context" lines.
        match (is_match, state.lines_after_match > 0) {
            // Scenario 1: Current line is a match.
            // This branch handles printing the matching line and its "before context".
            (true, _) => {
                // If we are starting a new printing block (i.e., not a continuation
                // from a previous match's context) and before context is requested,
                // print all lines currently in the before-context buffer.
                if !state.printing_block_active && before_context_num > 0 {
                    for (buffered_line_num, buffered_line) in state.before_context_buffer.drain(..)
                    {
                        print_line(buffered_line_num, &buffered_line, config.line_number);
                    }
                }

                // Clear the buffer after printing before-context lines, or if no
                // before-context was needed for this match.
                state.before_context_buffer.clear();

                // Print the matching line itself with highlighting.
                print_highlighted_line(state.line_count, &line, config.line_number, &search_regex);

                // Reset the counter for after-context lines and activate the printing block.
                state.lines_after_match = after_context_num;
                state.printing_block_active = true;
            }
            // Scenario 2: Current line is not a match, but we are still printing after-context lines.
            // This branch handles printing lines that follow a previous match as context.
            (false, true) => {
                // Print the current line as part of the after-context.
                print_line(state.line_count, &line, config.line_number);
                state.lines_after_match -= 1; // Decrement the after-context counter
                state.printing_block_active = true; // Stay in active printing block
            }
            // Scenario 3: Current line is neither a match nor part of active after-context.
            // This branch handles lines that are potential "before context" for future matches.
            (false, false) => {
                //Add this line to the before-context buffer.
                // `line` can be moved here as it's not used further in this iteration.
                state
                    .before_context_buffer
                    .push_back((state.line_count, line));

                // Ensure the buffer does not exceed the specified before-context size.
                // If it does, remove the oldest line from the front.
                if state.before_context_buffer.len() > before_context_num {
                    state.before_context_buffer.pop_front();
                }
                state.printing_block_active = false; // Not in an active printing block
            }
        }
    }

    Ok(())
}
