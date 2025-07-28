//! The main entry point for the `grep-rust` application.
//!
//! This module handles parsing command-line arguments, initializing the application
//! configuration, and executing the core search logic. It also manages error
//! handling for the application.

use clap::Parser;
use std::process;

pub mod config;
pub mod my_lib;
pub mod printer;

use crate::config::Config;
use crate::my_lib::run;

fn main() {
    // Parse command-line arguments into a Config struct.
    // Clap handles argument parsing and provides helpful error messages
    // if arguments are invalid or missing.
    let config = Config::parse();

    // Execute the main grep logic. If an error occurs during execution
    // (e.g., file not found, I/O error), print the error message to stderr
    // and exit the process with a non-zero status code.
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
