# minigrep

A simple command-line grep-like tool written in Rust. Searches for a query string in a file, with colored output and line numbers.

## Features

- Fast file searching using Rust's zero-copy string slicing
- Colored output with highlighted matches (red) and line numbers (green)
- Case-insensitive search via `-i`/`--ignore-case` flag
- Automatic `--help` and `--version` flags via clap

## Installation

```bash
# Build from source
cargo build --release

# The binary will be at target/release/minigrep
```

## Usage

```bash
# Basic case-sensitive search
cargo run -- "query" path/to/file.txt

# Case-insensitive search with short flag
cargo run -- "query" path/to/file.txt -i

# Case-insensitive search with long flag
cargo run -- "query" path/to/file.txt --ignore-case

# Show help
cargo run -- --help

# Show version
cargo run -- --version
```

## Examples

Create a test file:
```bash
echo -e "Rust:\nSafe, Fast, Productive.\nPick three." > test.txt
```

Search for "duct" (case-sensitive):
```bash
cargo run -- "duct" test.txt
# Output: 2: Safe, Fast, Productive.
#         (where "duct" is highlighted in red, "2:" is green)
```

Search for "rust" (case-insensitive):
```bash
cargo run -- "rust" test.txt -i
# Output: 1: Rust:
```

## Development

```bash
# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy

# Check compilation (faster than build)
cargo check
```

## Dependencies

- `clap` 4.6 — Command-line argument parsing via derive macro
- `colored` 3.1 — Terminal colors

## Architecture

- **`src/main.rs`** — Minimal entry point (~11 lines), parses arguments via clap, handles errors
- **`src/lib.rs`** — Core search logic with `Config`, `run()`, `search()`, `search_case_insensitive()`, and `format_line()`
