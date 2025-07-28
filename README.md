# Grep Rust

![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)

A basic command-line utility for searching text patterns in files, implemented in Rust. This project mimics some core functionalities of the classic `grep` command, offering features like case-insensitive search, line numbering, and context (lines before/after a match).

## Features

* **Pattern Search**: Find lines containing a specified text pattern.
* **File Input**: Search within any given text file.
* **Case-Insensitive Search (`-i`, `--ignore-case`)**: Perform searches regardless of character casing.
* **Line Numbering (`-l`, `--line-number`)**: Display line numbers alongside matching lines.
* **Context Lines**:
    * **Before Context (`-B NUM`, `--before-context NUM`)**: Show `NUM` lines preceding a match.
    * **After Context (`-A NUM`, `--after-context NUM`)**: Show `NUM` lines following a match.

## Installation

### Prerequisites

* [Rust](https://www.rust-lang.org/tools/install) programming language and Cargo package manager.

### Building from Source

1.  **Clone the repository:**
    ```bash
    git clone [https://github.com/YOUR_USERNAME/grep-rust.git](https://github.com/YOUR_USERNAME/grep-rust.git)
    cd grep-rust
    ```
2.  **Build the project:**
    ```bash
    cargo build --release
    ```
    This will create an executable file in `target/release/grep-rust` (or `target/release/grep-rust.exe` on Windows).

3.  **Add to your PATH (Optional):**
    To run `grep-rust` from any directory, you can add `~/.cargo/bin` to your system's PATH, or copy the compiled binary to a directory already in your PATH (e.g., `/usr/local/bin` on Linux/macOS).
    ```bash
    # On Linux/macOS, after building:
    cp target/release/grep-rust /usr/local/bin/
    ```

## Usage

The basic syntax is `grep-rust <QUERY> <FILE_PATH> [OPTIONS]`.

```bash
# Basic search for "rust" in myfile.txt
./target/release/grep-rust rust myfile.txt

# Case-insensitive search for "Rust" in code.txt with line numbers
./target/release/grep-rust -i -l Rust code.txt

# Search for "error" in logs.txt, showing 3 lines before and 2 lines after each match
./target/release/grep-rust -B 3 -A 2 error logs.txt

# Combine options
./target/release/grep-rust -i -l -B 1 -A 1 warning system.log