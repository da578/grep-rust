//! Contains the core logic for the `grep-rust` application.
//!
//! This module defines the `GrepState` to manage the search process's internal
//! state and the `run` function, which orchestrates file reading, pattern
//! matching, and context handling.

use crate::config::Config;
use crate::printer::{print_line, print_search_info};

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
    /// printing a match or its context lines).
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

    // Prepare the query string based on case-insensitivity option.
    let processed_query = if config.ignore_case {
        config.query.to_lowercase()
    } else {
        config.query
    };

    // Open the file and create a buffered reader for efficient line-by-line reading.
    let file = File::open(config.file_path)?;
    let reader = BufReader::new(file);

    let mut state = GrepState::new();

    // Iterate through each line of the file.
    for line_result in reader.lines() {
        state.line_count += 1; // Increment line count for each line processed
        let line = line_result?; // Get the current line content

        // Prepare the line content for matching based on case-insensitivity.
        let processed_line = if config.ignore_case {
            line.to_lowercase()
        } else {
            line.clone() // Clone to keep the original `line` available for printing
        };

        // Check if the current line matches the processed query.
        let is_match = processed_line.contains(&processed_query);

        // Use a match statement to handle different scenarios:
        // 1. Current line is a match.
        // 2. Current line is not a match, but is part of "after context".
        // 3. Current line is neither a match nor part of "after context".
        match (is_match, state.lines_after_match > 0) {
            // Scenario 1: Current line is a match.
            // If we are starting a new printing block and before context is requested,
            // print all lines currently in the before-context buffer.
            (true, _) => {
                if !state.printing_block_active && before_context_num > 0 {
                    for (buffered_line_num, buffered_line) in state.before_context_buffer.drain(..)
                    {
                        print_line(buffered_line_num, &buffered_line, config.line_number);
                    }
                }

                state.before_context_buffer.clear(); // Clear buffer after printing or if no before context needed

                // Clear buffer after printing or if no before context needed
                print_line(state.line_count, &line, config.line_number);

                // Reset the counter for after-context lines and activate the printing block.
                state.lines_after_match = after_context_num;
                state.printing_block_active = true;
            }
            (false, true) => {
                // Scenario 2: Current line is not a match, but we are still printing after-context lines.
                // Print the current line as part of the after-context.
                print_line(state.line_count, &line, config.line_number);
                state.lines_after_match -= 1; // Decrement the after-context counter
                state.printing_block_active = true; // Stay in active printing block
            }
            (false, false) => {
                // Scenario 3: Current line is neither a match nor part of active after-context.
                // Add this line to the before-context buffer as a potential context line for future matches.
                state
                    .before_context_buffer
                    .push_back((state.line_count, line));

                // Ensure the buffer does not exceed the specified before-context size.
                if state.before_context_buffer.len() > before_context_num {
                    state.before_context_buffer.pop_front();
                }
                state.printing_block_active = false; // Not in an active printing block
            }
        }
    }

    Ok(())
}